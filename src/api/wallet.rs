use crate::engine::{BitcoinClient, ReqPath, RpcResponse};
use async_trait::async_trait;
use serde_json::Value;

use super::{BitcoinCLI, IWallet};

#[async_trait]
impl IWallet for BitcoinCLI {
	async fn createwallet(&self, wallet_name: &str) -> Value {
		let client = BitcoinClient::new();
		let req_path = ReqPath::new(&client.config.bitcoin_node, "/");

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "createwallet", vec![wallet_name.into()])
			.await
			.unwrap();

		rpc_response.result
	}

	async fn getwalletinfo(&self, wallet_name: &str) -> Value {
		let client = BitcoinClient::new();
		let endpoint = "wallet/".to_string() + wallet_name;
		let req_path = ReqPath::new(&client.config.bitcoin_node, &endpoint);

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "getwalletinfo", vec![])
			.await
			.unwrap();

		rpc_response.result
	}

	async fn getbalance(&self, wallet_name: &str) -> Value {
		let client = BitcoinClient::new();
		let endpoint = "wallet/".to_string() + wallet_name;
		let req_path = ReqPath::new(&client.config.bitcoin_node, &endpoint);

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "getbalance", vec![])
			.await
			.unwrap();

		rpc_response.result
	}

	async fn getnewaddress(
		&self,
		wallet_name: &str,
		label: Option<String>,
		address_type: Option<String>,
	) -> Value {
		let client = BitcoinClient::new();

		let endpoint = "wallet/".to_string() + wallet_name;

		let req_path = ReqPath::new(&client.config.bitcoin_node, &endpoint);

		let label = label.unwrap_or("".into());
		let address_type = address_type.unwrap_or("legacy".into());

		let rpc_response = client
			.send_request::<RpcResponse>(
				&req_path,
				"getnewaddress",
				vec![label.into(), address_type.into()],
			)
			.await
			.unwrap();

		rpc_response.result
	}
}
