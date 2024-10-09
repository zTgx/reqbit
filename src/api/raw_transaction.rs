use crate::engine::{BitcoinClient, RpcResponse};
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

		let mut params = vec![json!(inputs), json!(outputs)];
		if let Some(lt) = locktime {
			params.push(lt.into());
		}

		println!("Request body: {}", serde_json::to_string_pretty(&params).unwrap());

		let rpc_response = client
			.send_request::<RpcResponse>(None, "createrawtransaction", params)
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

		let mut params = vec![hexstring.into(), json!(privkeys)];
		if let Some(sighash) = sighash_type {
			params.push(sighash.into());
		}

		let rpc_response = client
			.send_request::<RpcResponse>(None, "signrawtransactionwithkey", params)
			.await
			.unwrap();

		rpc_response.result
	}

	async fn sendrawtransaction(&self, hexstring: &str) -> Value {
		let client = BitcoinClient::new();

		let rpc_response = client
			.send_request::<RpcResponse>(None, "sendrawtransaction", vec![hexstring.into()])
			.await
			.unwrap();

		rpc_response.result
	}

	async fn getrawtransaction(
		&self,
		txid: &str,
		verbose: Option<bool>,
		blockhash: Option<&str>,
	) -> Value {
		let client = BitcoinClient::new();

		let mut params = vec![txid.into()];
		if let Some(v) = verbose {
			params.push(json!(v));
			if let Some(bh) = blockhash {
				params.push(bh.into());
			}
		}

		let rpc_response = client
			.send_request::<RpcResponse>(None, "getrawtransaction", params)
			.await
			.unwrap();

		rpc_response.result
	}

	async fn decoderawtransaction(&self, hexstring: &str, iswitness: Option<bool>) -> Value {
		let client = BitcoinClient::new();

		let mut params = vec![hexstring.into()];
		if let Some(witness) = iswitness {
			params.push(json!(witness));
		}

		let rpc_response = client
			.send_request::<RpcResponse>(None, "decoderawtransaction", params)
			.await
			.unwrap();

		rpc_response.result
	}
}
