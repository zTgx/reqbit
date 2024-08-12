use crate::engine::{BitcoinClient, RpcResponse};
use async_trait::async_trait;
use serde_json::Value;

use super::{BitcoinCLI, IWallet};

#[async_trait]
impl IWallet for BitcoinCLI {
	async fn createwallet(&self, wallet_name: &str) -> Value {
		let client = BitcoinClient::new();

		let rpc_response = client
			.send_request::<RpcResponse>("createwallet", vec![wallet_name.into()])
			.await
			.unwrap();

		rpc_response.result
	}
}
