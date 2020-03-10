use jsonrpc_client::{jsonrpc, serialize_parameters};

jsonrpc!(pub struct Rpc {
    pub fn get_data(&self, _id: u64) -> Option<Vec<u8>>;
});

pub struct RpcClient {
    rpc: Rpc,
}

impl RpcClient {
    pub fn new(uri: &str) -> Self {
        let client = reqwest::blocking::Client::builder()
            .gzip(true)
            .timeout(::std::time::Duration::from_secs(30))
            .build()
            .expect("reqwest client");
        Self {
            rpc: Rpc::new(uri, client),
        }
    }

    pub fn get_data(&self, id: u64) -> Option<Vec<u8>> {
        self.rpc.get_data(id).expect("rpc call")
    }
}

fn main() {
    RpcClient::new("localhost");
}
