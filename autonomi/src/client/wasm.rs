use libp2p::Multiaddr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Client(super::Client);

#[wasm_bindgen]
pub struct ChunkAddr(xor_name::XorName);

#[wasm_bindgen]
pub struct DataAddr(xor_name::XorName);
#[wasm_bindgen]
impl DataAddr {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        crate::client::address::xorname_to_str(self.0)
    }
}

#[wasm_bindgen]
pub struct AttoTokens(sn_evm::AttoTokens);
#[wasm_bindgen]
impl AttoTokens {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub async fn connect(peers: Vec<String>) -> Result<Client, JsError> {
        let peers = peers
            .into_iter()
            .map(|peer| peer.parse())
            .collect::<Result<Vec<Multiaddr>, _>>()?;

        let client = super::Client::connect(&peers).await?;

        Ok(Client(client))
    }

    #[wasm_bindgen(js_name = putChunk)]
    pub async fn put_chunk(&self, _data: Vec<u8>, _wallet: Wallet) -> Result<ChunkAddr, JsError> {
        unimplemented!()
    }

    #[wasm_bindgen(js_name = getChunk)]
    pub async fn get_chunk(&self, addr: ChunkAddr) -> Result<Vec<u8>, JsError> {
        let chunk = self.0.fetch_chunk(addr.0).await?;
        Ok(chunk.value().to_vec())
    }

    #[wasm_bindgen(js_name = putData)]
    pub async fn put_data(&self, data: Vec<u8>, wallet: Wallet) -> Result<DataAddr, JsError> {
        let data = crate::Bytes::from(data);
        let xorname = self.0.put(data, &wallet.0).await?;
        Ok(DataAddr(xorname))
    }

    #[wasm_bindgen(js_name = getData)]
    pub async fn get_data(&self, addr: DataAddr) -> Result<Vec<u8>, JsError> {
        let data = self.0.get(addr.0).await?;
        Ok(data.to_vec())
    }

    #[wasm_bindgen]
    pub async fn cost(&self, data: Vec<u8>) -> Result<AttoTokens, JsValue> {
        let data = crate::Bytes::from(data);
        let cost = self.0.cost(data).await.map_err(|e| JsError::from(e))?;

        Ok(AttoTokens(cost))
    }
}

#[wasm_bindgen]
pub struct Wallet(evmlib::wallet::Wallet);

/// Get a funded wallet for testing. This either uses a default private key or the `EVM_PRIVATE_KEY`
/// environment variable that was used during the build process of this library.
#[wasm_bindgen(js_name = getFundedWallet)]
pub fn funded_wallet() -> Wallet {
    let network = evmlib::utils::evm_network_from_env().expect("network init from env");

    let private_key = option_env!("EVM_PRIVATE_KEY")
        .unwrap_or_else(|| "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80");

    Wallet(
        evmlib::wallet::Wallet::new_from_private_key(network, private_key)
            .expect("Invalid private key"),
    )
}

/// Enable tracing logging in the console.
///
/// A level could be passed like `trace` or `warn`. Or set for a specific module/crate
/// with `sn_networking=trace,autonomi=info`.
#[wasm_bindgen(js_name = logInit)]
pub fn log_init(directive: String) {
    use tracing_subscriber::prelude::*;

    console_error_panic_hook::set_once();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false) // Only partially supported across browsers
        .without_time() // std::time is not available in browsers
        .with_writer(tracing_web::MakeWebConsoleWriter::new()); // write events to the console
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(tracing_subscriber::EnvFilter::new(directive))
        .init();
}
