use crate::{
	engine::{BitcoinClient, RpcResponse},
	INetwork, ReqBit, Result,
};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
impl INetwork for ReqBit {
	async fn getnetworkinfo(&self) -> Result<Value> {
		let client = BitcoinClient::new();
		match client.send_request::<RpcResponse>(None, "getnetworkinfo", vec![]).await {
			Ok(res) => Ok(res.result),
			Err(err) => Err(err),
		}
	}
}
