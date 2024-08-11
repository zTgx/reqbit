use serde_json::Value;
use async_trait::async_trait;
use crate::engine::{BitcoinClient, RpcResponse};

use super::{BitcoinCLI, IMining};

#[async_trait]
impl IMining for BitcoinCLI {
	async fn get_mining_info(&self) -> Value {
		let client = BitcoinClient::new();

		let rpc_response = client.send_request::<RpcResponse>("getmininginfo", vec![]).await.unwrap();

		rpc_response.result
	}
}
