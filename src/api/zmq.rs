use async_trait::async_trait;
use serde_json::Value;
use crate::IZmq;
use crate::api::bitcoin_client::{BitcoinClient, ReqPath, RpcResponse};

#[async_trait]
impl IZmq for crate::ReqBit {
    async fn getzmqnotifications(&self) -> Value {
        let client = BitcoinClient::new();
        let req_path = ReqPath::new(&client.config.bitcoin_node, "");

        let rpc_response = client
            .send_request::<RpcResponse>(&req_path, "getzmqnotifications", vec![])
            .await
            .unwrap();

        rpc_response.result
    }
}
