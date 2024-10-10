use crate::{
	engine::{BitcoinClient, RpcResponse},
	IBlockchain, ReqBit, Result,
};
use async_trait::async_trait;
use serde_json::{json, Value};

#[async_trait]
impl IBlockchain for ReqBit {
	async fn getblock(&self, blockhash: &str, verbosity: Option<u8>) -> Result<Value> {
		let client = BitcoinClient::new();

		let mut params = vec![json!(blockhash)];
		if let Some(verb) = verbosity {
			params.push(json!(verb));
		}

		match client.send_request::<RpcResponse>(None, "getblock", params).await {
			Ok(res) => Ok(res.result),
			Err(err) => Err(err),
		}
	}

	async fn getblockhash(&self, height: u32) -> Result<Value> {
		let client = BitcoinClient::new();
		match client
			.send_request::<RpcResponse>(None, "getblockhash", vec![json!(height)])
			.await
		{
			Ok(res) => Ok(res.result),
			Err(err) => Err(err),
		}
	}

	async fn getblockchaininfo(&self) -> Value {
		let client = BitcoinClient::new();

		let rpc_response = client
			.send_request::<RpcResponse>(None, "getblockchaininfo", vec![])
			.await
			.unwrap();

		rpc_response.result
	}
}
