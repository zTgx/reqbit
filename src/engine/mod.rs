pub mod config;

use self::config::ReqbitConfig;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ReqBody {
	pub jsonrpc: String,
	pub id: String,
	pub method: String,
	pub params: Vec<Value>,
}

#[derive(Deserialize, Debug)]
pub struct RpcResponse {
	pub result: Value,
	pub error: Option<String>,
	pub id: String,
}

#[macro_export]
macro_rules! rpc_request {
	($method:expr, $params:expr) => {
		ReqBody {
			jsonrpc: String::from("1.0"),
			id: String::from("1"),
			method: String::from($method),
			params: $params.into(),
		}
	};
}

pub struct BitcoinClient {
	config: ReqbitConfig,
	client: reqwest::Client,
}

impl BitcoinClient {
	pub fn new() -> Self {
		let config = ReqbitConfig::new().unwrap();

		BitcoinClient { config, client: reqwest::Client::new() }
	}

	pub async fn send_request<T: serde::de::DeserializeOwned>(
		&self,
		method: &str,
		params: Vec<serde_json::Value>,
	) -> Result<T, reqwest::Error> {
		let req_body = rpc_request!(method, params);

		let response = self
			.client
			.post(&self.config.bitcoin_node)
			.header("Authorization", format!("Basic {}", self.config.get_auth()))
			.json(&req_body)
			.send()
			.await?;

		response.json::<T>().await
	}
}
