use crate::engine::{BitcoinClient, ReqPath, RpcResponse};
use async_trait::async_trait;
use serde_json::{json, Value};

use crate::{IRawTransaction, ReqBit};

#[async_trait]
impl IRawTransaction for ReqBit {
	async fn createrawtransaction(
		&self,
		inputs: Vec<Value>,
		outputs: Vec<Value>,
		locktime: Option<u32>,
	) -> Value {
		let client = BitcoinClient::new();
		let req_path = ReqPath::new(&client.config.bitcoin_node, "");

		let mut params = vec![json!(inputs), json!(outputs)];
		if let Some(lt) = locktime {
			params.push(lt.into());
		}

		println!("URL: {}", req_path.to_string());
		println!("Request body: {}", serde_json::to_string_pretty(&params).unwrap());

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "createrawtransaction", params)
			.await
			.unwrap();

		rpc_response.result
	}

	async fn signrawtransactionwithkey(
		&self,
		hexstring: &str,
		privkeys: Vec<String>,
		sighash_type: Option<&str>,
	) -> Value {
		let client = BitcoinClient::new();
		let req_path = ReqPath::new(&client.config.bitcoin_node, "");

		let mut params = vec![hexstring.into(), json!(privkeys)];
		if let Some(sighash) = sighash_type {
			params.push(sighash.into());
		}

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "signrawtransactionwithkey", params)
			.await
			.unwrap();

		rpc_response.result
	}

	async fn sendrawtransaction(&self, hexstring: &str) -> Value {
		let client = BitcoinClient::new();
		let req_path = ReqPath::new(&client.config.bitcoin_node, "");

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "sendrawtransaction", vec![hexstring.into()])
			.await
			.unwrap();

		rpc_response.result
	}
}
