package autonomi

/*
#cgo LDFLAGS: -L../target/release -lautonomi_ffi
#include "../ffi/autonomi.h"
#include <stdlib.h>
*/
import "C"
import (
	"fmt"
	"runtime"
	"unsafe"
)

// ErrorCode represents FFI error codes
type ErrorCode int

const (
	Success         ErrorCode = C.Success
	InvalidArgument ErrorCode = C.InvalidArgument
	ConnectionError ErrorCode = C.ConnectionError
	RuntimeError    ErrorCode = C.RuntimeError
	InvalidUtf8     ErrorCode = C.InvalidUtf8
	NullPointer     ErrorCode = C.NullPointer
)

// Client represents a connection to the Autonomi network
type Client struct {
	ptr *C.FfiClient
}

// Wallet represents an EVM wallet for payments
type Wallet struct {
	ptr *C.FfiWallet
}

// Error represents an error from the FFI layer
type Error struct {
	Code    ErrorCode
	Message string
}

func (e *Error) Error() string {
	return fmt.Sprintf("autonomi error %d: %s", e.Code, e.Message)
}

// handleResult converts FFI result to Go error
func handleResult(result *C.FfiResult) error {
	if result.error_code == C.Success {
		return nil
	}

	var message string
	if result.error_message != nil {
		message = C.GoString(result.error_message)
		C.autonomi_free_string(result.error_message)
	}

	return &Error{
		Code:    ErrorCode(result.error_code),
		Message: message,
	}
}

// InitLocal creates a client connected to the local network
func InitLocal() (*Client, error) {
	var result C.FfiResult
	ptr := C.autonomi_client_init_local(&result)
	
	if err := handleResult(&result); err != nil {
		return nil, err
	}

	if ptr == nil {
		return nil, &Error{Code: RuntimeError, Message: "failed to initialize client"}
	}

	client := &Client{ptr: ptr}
	runtime.SetFinalizer(client, (*Client).free)
	return client, nil
}

// Init creates a client connected to the main network
func Init() (*Client, error) {
	var result C.FfiResult
	ptr := C.autonomi_client_init(&result)
	
	if err := handleResult(&result); err != nil {
		return nil, err
	}

	if ptr == nil {
		return nil, &Error{Code: RuntimeError, Message: "failed to initialize client"}
	}

	client := &Client{ptr: ptr}
	runtime.SetFinalizer(client, (*Client).free)
	return client, nil
}

// free releases the client resources
func (c *Client) free() {
	if c.ptr != nil {
		C.autonomi_client_free(c.ptr)
		c.ptr = nil
	}
}

// Close explicitly releases client resources
func (c *Client) Close() {
	c.free()
	runtime.SetFinalizer(c, nil)
}

// DataPut uploads data to the network and returns the address
func (c *Client) DataPut(data []byte, wallet *Wallet) (string, error) {
	if c.ptr == nil {
		return "", &Error{Code: NullPointer, Message: "client is closed"}
	}
	if wallet == nil || wallet.ptr == nil {
		return "", &Error{Code: NullPointer, Message: "wallet is nil"}
	}

	var result C.FfiResult
	var dataPtr *C.uint8_t
	if len(data) > 0 {
		dataPtr = (*C.uint8_t)(unsafe.Pointer(&data[0]))
	}

	addressPtr := C.autonomi_client_data_put(
		c.ptr,
		dataPtr,
		C.size_t(len(data)),
		wallet.ptr,
		&result,
	)

	if err := handleResult(&result); err != nil {
		return "", err
	}

	if addressPtr == nil {
		return "", &Error{Code: RuntimeError, Message: "failed to get address"}
	}

	address := C.GoString(addressPtr)
	C.autonomi_free_string(addressPtr)
	return address, nil
}

// DataGet retrieves data from the network using an address
func (c *Client) DataGet(address string) ([]byte, error) {
	if c.ptr == nil {
		return nil, &Error{Code: NullPointer, Message: "client is closed"}
	}

	var result C.FfiResult
	var dataLen C.size_t

	cAddress := C.CString(address)
	defer C.free(unsafe.Pointer(cAddress))

	dataPtr := C.autonomi_client_data_get(
		c.ptr,
		cAddress,
		&result,
		&dataLen,
	)

	if err := handleResult(&result); err != nil {
		return nil, err
	}

	if dataPtr == nil {
		return nil, &Error{Code: RuntimeError, Message: "failed to get data"}
	}

	// Copy the data to a Go slice
	data := C.GoBytes(unsafe.Pointer(dataPtr), C.int(dataLen))
	
	// Free the C memory
	C.autonomi_free_data(dataPtr, dataLen)

	return data, nil
}

// NewWalletFromPrivateKeyLocal creates a wallet from a private key for local network
func NewWalletFromPrivateKeyLocal(privateKey string) (*Wallet, error) {
	var result C.FfiResult

	cPrivateKey := C.CString(privateKey)
	defer C.free(unsafe.Pointer(cPrivateKey))

	ptr := C.autonomi_wallet_from_private_key_local(cPrivateKey, &result)

	if err := handleResult(&result); err != nil {
		return nil, err
	}

	if ptr == nil {
		return nil, &Error{Code: RuntimeError, Message: "failed to create wallet"}
	}

	wallet := &Wallet{ptr: ptr}
	runtime.SetFinalizer(wallet, (*Wallet).free)
	return wallet, nil
}

// free releases the wallet resources
func (w *Wallet) free() {
	if w.ptr != nil {
		C.autonomi_wallet_free(w.ptr)
		w.ptr = nil
	}
}

// Close explicitly releases wallet resources
func (w *Wallet) Close() {
	w.free()
	runtime.SetFinalizer(w, nil)
}