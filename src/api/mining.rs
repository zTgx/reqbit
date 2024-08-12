use crate::engine::{BitcoinClient, ReqPath, RpcResponse};
use async_trait::async_trait;
use serde_json::{json, Value};

use super::{BitcoinCLI, IMining};

#[async_trait]
impl IMining for BitcoinCLI {
	async fn get_block_template(&self, template: &str) -> Value {
		let client = BitcoinClient::new();
		let req_path = ReqPath::new(&client.config.bitcoin_node, "/");

		let rules = json!({"rules": [template]});

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "getblocktemplate", vec![rules])
			.await
			.unwrap();

		rpc_response.result
	}

	async fn get_mining_info(&self) -> Value {
		let client = BitcoinClient::new();
		let req_path = ReqPath::new(&client.config.bitcoin_node, "/");

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "getmininginfo", vec![])
			.await
			.unwrap();

		rpc_response.result
	}

	async fn getnetworkhashps(&self, nblocks: Option<i32>, height: Option<u32>) -> Value {
		let client = BitcoinClient::new();
		let req_path = ReqPath::new(&client.config.bitcoin_node, "/");

		let nblocks = nblocks.unwrap_or(120);
		let height = height.unwrap_or(1);

		let rpc_response = client
			.send_request::<RpcResponse>(
				&req_path,
				"getnetworkhashps",
				vec![nblocks.into(), height.into()],
			)
			.await
			.unwrap();

		rpc_response.result
	}
}
