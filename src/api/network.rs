use crate::{
	engine::{BitcoinClient, ReqPath, RpcResponse},
	INetwork, ReqBit, Result,
};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
impl INetwork for ReqBit {
	async fn getnetworkinfo(&self) -> Result<Value> {
		let client = BitcoinClient::new();
		let req_path = ReqPath::new(&client.config.bitcoin_node, "");

		match client.send_request::<RpcResponse>(&req_path, "getnetworkinfo", vec![]).await {
			Ok(res) => Ok(res.result),
			Err(err) => Err(err),
		}
	}
}
