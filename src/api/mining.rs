use crate::engine::{BitcoinClient, RpcResponse};
use async_trait::async_trait;
use serde_json::{json, Value};

use super::{BitcoinCLI, IMining};

#[async_trait]
impl IMining for BitcoinCLI {
	async fn get_block_template(&self, template: &str) -> Value {
		let client = BitcoinClient::new();

		let rules = json!({"rules": [template]});

		let rpc_response = client
			.send_request::<RpcResponse>("getblocktemplate", vec![rules])
			.await
			.unwrap();

		rpc_response.result
	}

	async fn get_mining_info(&self) -> Value {
		let client = BitcoinClient::new();

		let rpc_response =
			client.send_request::<RpcResponse>("getmininginfo", vec![]).await.unwrap();

		rpc_response.result
	}
}
