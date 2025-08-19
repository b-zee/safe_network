use autonomi::{Client, Wallet, Network, Bytes};
use autonomi::client::payment::PaymentOption;
use autonomi::data::private::DataMapChunk;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Opaque type for the Client
pub struct FfiClient {
    client: Client,
    runtime: Arc<Runtime>,
}

/// Opaque type for the Wallet
pub struct FfiWallet {
    wallet: Wallet,
}

/// Error codes for FFI operations
#[repr(C)]
pub enum ErrorCode {
    Success = 0,
    InvalidArgument = 1,
    ConnectionError = 2,
    RuntimeError = 3,
    InvalidUtf8 = 4,
    NullPointer = 5,
}

/// Result structure for FFI operations
#[repr(C)]
pub struct FfiResult {
    error_code: ErrorCode,
    error_message: *mut c_char,
}

impl FfiResult {
    fn success() -> Self {
        FfiResult {
            error_code: ErrorCode::Success,
            error_message: ptr::null_mut(),
        }
    }

    fn error(code: ErrorCode, message: String) -> Self {
        let c_message = CString::new(message).unwrap_or_else(|_| CString::new("Error message contained null byte").unwrap());
        FfiResult {
            error_code: code,
            error_message: c_message.into_raw(),
        }
    }
}

/// Initialize a client connected to the local network
/// Returns a pointer to the client or null on error
#[no_mangle]
pub extern "C" fn autonomi_client_init_local(result: *mut FfiResult) -> *mut FfiClient {
    if result.is_null() {
        return ptr::null_mut();
    }

    let runtime = match Runtime::new() {
        Ok(rt) => Arc::new(rt),
        Err(e) => {
            unsafe {
                *result = FfiResult::error(ErrorCode::RuntimeError, format!("Failed to create runtime: {}", e));
            }
            return ptr::null_mut();
        }
    };

    let client = match runtime.block_on(Client::init_local()) {
        Ok(client) => client,
        Err(e) => {
            unsafe {
                *result = FfiResult::error(ErrorCode::ConnectionError, format!("Failed to connect to local network: {}", e));
            }
            return ptr::null_mut();
        }
    };

    let ffi_client = Box::new(FfiClient {
        client,
        runtime,
    });

    unsafe {
        *result = FfiResult::success();
    }

    Box::into_raw(ffi_client)
}

/// Initialize a client connected to the main network
#[no_mangle]
pub extern "C" fn autonomi_client_init(result: *mut FfiResult) -> *mut FfiClient {
    if result.is_null() {
        return ptr::null_mut();
    }

    let runtime = match Runtime::new() {
        Ok(rt) => Arc::new(rt),
        Err(e) => {
            unsafe {
                *result = FfiResult::error(ErrorCode::RuntimeError, format!("Failed to create runtime: {}", e));
            }
            return ptr::null_mut();
        }
    };

    let client = match runtime.block_on(Client::init()) {
        Ok(client) => client,
        Err(e) => {
            unsafe {
                *result = FfiResult::error(ErrorCode::ConnectionError, format!("Failed to connect to network: {}", e));
            }
            return ptr::null_mut();
        }
    };

    let ffi_client = Box::new(FfiClient {
        client,
        runtime,
    });

    unsafe {
        *result = FfiResult::success();
    }

    Box::into_raw(ffi_client)
}

/// Free a client instance
#[no_mangle]
pub extern "C" fn autonomi_client_free(client: *mut FfiClient) {
    if !client.is_null() {
        unsafe {
            let _ = Box::from_raw(client);
        }
    }
}

/// Upload data and return the address as a hex string
#[no_mangle]
pub extern "C" fn autonomi_client_data_put(
    client: *mut FfiClient,
    data: *const u8,
    data_len: usize,
    wallet: *mut FfiWallet,
    result: *mut FfiResult,
) -> *mut c_char {
    if result.is_null() {
        return ptr::null_mut();
    }

    if client.is_null() || data.is_null() || wallet.is_null() {
        unsafe {
            *result = FfiResult::error(ErrorCode::NullPointer, "Null pointer provided".to_string());
        }
        return ptr::null_mut();
    }

    let client = unsafe { &*client };
    let wallet = unsafe { &*wallet };
    let data_slice = unsafe { std::slice::from_raw_parts(data, data_len) };
    let data_vec = data_slice.to_vec();

    let (_cost, data_map_chunk) = match client.runtime.block_on(
        client.client.data_put(Bytes::from(data_vec), PaymentOption::Wallet(wallet.wallet.clone()))
    ) {
        Ok(result) => result,
        Err(e) => {
            unsafe {
                *result = FfiResult::error(ErrorCode::RuntimeError, format!("Failed to upload data: {}", e));
            }
            return ptr::null_mut();
        }
    };

    let address_hex = data_map_chunk.address();
    let c_address = match CString::new(address_hex) {
        Ok(s) => s,
        Err(e) => {
            unsafe {
                *result = FfiResult::error(ErrorCode::InvalidUtf8, format!("Invalid address string: {}", e));
            }
            return ptr::null_mut();
        }
    };

    unsafe {
        *result = FfiResult::success();
    }

    c_address.into_raw()
}

/// Get data from the network using a hex address
#[no_mangle]
pub extern "C" fn autonomi_client_data_get(
    client: *mut FfiClient,
    address_hex: *const c_char,
    result: *mut FfiResult,
    data_len: *mut usize,
) -> *mut u8 {
    if result.is_null() || data_len.is_null() {
        return ptr::null_mut();
    }

    if client.is_null() || address_hex.is_null() {
        unsafe {
            *result = FfiResult::error(ErrorCode::NullPointer, "Null pointer provided".to_string());
        }
        return ptr::null_mut();
    }

    let client = unsafe { &*client };
    
    let address_str = unsafe {
        match CStr::from_ptr(address_hex).to_str() {
            Ok(s) => s,
            Err(e) => {
                *result = FfiResult::error(ErrorCode::InvalidUtf8, format!("Invalid UTF-8 in address: {}", e));
                return ptr::null_mut();
            }
        }
    };

    let data_map = match DataMapChunk::from_hex(address_str) {
        Ok(dmc) => dmc,
        Err(e) => {
            unsafe {
                *result = FfiResult::error(ErrorCode::InvalidArgument, format!("Invalid data map address: {}", e));
            }
            return ptr::null_mut();
        }
    };

    let data = match client.runtime.block_on(client.client.data_get(&data_map)) {
        Ok(data) => data,
        Err(e) => {
            unsafe {
                *result = FfiResult::error(ErrorCode::RuntimeError, format!("Failed to get data: {}", e));
            }
            return ptr::null_mut();
        }
    };

    let data_bytes = data.to_vec();
    let len = data_bytes.len();
    let data_ptr = data_bytes.as_ptr() as *mut u8;
    std::mem::forget(data_bytes);

    unsafe {
        *data_len = len;
        *result = FfiResult::success();
    }

    data_ptr
}

/// Create a wallet from a private key for local network
#[no_mangle]
pub extern "C" fn autonomi_wallet_from_private_key_local(
    private_key: *const c_char,
    result: *mut FfiResult,
) -> *mut FfiWallet {
    if result.is_null() {
        return ptr::null_mut();
    }

    if private_key.is_null() {
        unsafe {
            *result = FfiResult::error(ErrorCode::NullPointer, "Null private key provided".to_string());
        }
        return ptr::null_mut();
    }

    let key_str = unsafe {
        match CStr::from_ptr(private_key).to_str() {
            Ok(s) => s,
            Err(e) => {
                *result = FfiResult::error(ErrorCode::InvalidUtf8, format!("Invalid UTF-8 in private key: {}", e));
                return ptr::null_mut();
            }
        }
    };

    let evm_network = match Network::new(true) {
        Ok(network) => network,
        Err(e) => {
            unsafe {
                *result = FfiResult::error(ErrorCode::RuntimeError, format!("Failed to create local EVM network: {}", e));
            }
            return ptr::null_mut();
        }
    };
    
    let wallet = match Wallet::new_from_private_key(evm_network, key_str) {
        Ok(w) => w,
        Err(e) => {
            unsafe {
                *result = FfiResult::error(ErrorCode::InvalidArgument, format!("Invalid private key: {}", e));
            }
            return ptr::null_mut();
        }
    };

    let ffi_wallet = Box::new(FfiWallet { wallet });

    unsafe {
        *result = FfiResult::success();
    }

    Box::into_raw(ffi_wallet)
}

/// Free a wallet instance
#[no_mangle]
pub extern "C" fn autonomi_wallet_free(wallet: *mut FfiWallet) {
    if !wallet.is_null() {
        unsafe {
            let _ = Box::from_raw(wallet);
        }
    }
}

/// Free a C string allocated by this library
#[no_mangle]
pub extern "C" fn autonomi_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

/// Free data allocated by this library
#[no_mangle]
pub extern "C" fn autonomi_free_data(data: *mut u8, len: usize) {
    if !data.is_null() && len > 0 {
        unsafe {
            let _ = Vec::from_raw_parts(data, len, len);
        }
    }
}