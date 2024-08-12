pub mod config;

use self::config::ReqbitConfig;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

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

pub struct ReqPath {
	pub base_url: String,
	pub endpoint: String,
}

impl ReqPath {
	pub fn new(base_url: &str, endpoint: &str) -> Self {
		ReqPath { base_url: base_url.to_string(), endpoint: endpoint.to_string() }
	}
}

impl fmt::Display for ReqPath {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", self.base_url, self.endpoint)
	}
}

pub struct BitcoinClient {
	pub config: ReqbitConfig,
	client: reqwest::Client,
}

impl BitcoinClient {
	pub fn new() -> Self {
		let config = ReqbitConfig::new().unwrap();

		BitcoinClient { config, client: reqwest::Client::new() }
	}

	pub async fn send_request<T: serde::de::DeserializeOwned>(
		&self,
		req_path: &ReqPath,
		method: &str,
		params: Vec<serde_json::Value>,
	) -> Result<T, reqwest::Error> {
		let req_body = rpc_request!(method, params);
		let url = req_path.to_string();
		println!(">> url: {}", url);

		let response = self
			.client
			.post(&url)
			.header("Authorization", format!("Basic {}", self.config.get_auth()))
			.json(&req_body)
			.send()
			.await?;

		response.json::<T>().await
	}
}
