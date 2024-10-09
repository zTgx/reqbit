use crate::{
	engine::{BitcoinClient, RpcResponse},
	IBlockchain, ReqBit,
};
use async_trait::async_trait;
use serde_json::{json, Value};

#[async_trait]
impl IBlockchain for ReqBit {
	async fn getblock(&self, blockhash: &str, verbosity: Option<u8>) -> Value {
		let client = BitcoinClient::new();

		let mut params = vec![json!(blockhash)];
		if let Some(verb) = verbosity {
			params.push(json!(verb));
		}

		let rpc_response =
			client.send_request::<RpcResponse>(None, "getblock", params).await.unwrap();

		rpc_response.result
	}

	async fn getblockhash(&self, height: u32) -> Value {
		let client = BitcoinClient::new();

		let params = vec![json!(height)];

		let rpc_response =
			client.send_request::<RpcResponse>(None, "getblockhash", params).await.unwrap();

		rpc_response.result
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
