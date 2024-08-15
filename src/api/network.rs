use crate::engine::{BitcoinClient, ReqPath, RpcResponse};
use async_trait::async_trait;
use serde_json::Value;

use crate::{INetwork, ReqBit};

#[async_trait]
impl INetwork for ReqBit {
	async fn getnetworkinfo(&self) -> Value {
		let client = BitcoinClient::new();
		let req_path = ReqPath::new(&client.config.bitcoin_node, "");

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "getnetworkinfo", vec![])
			.await
			.unwrap();

		rpc_response.result
	}
}
