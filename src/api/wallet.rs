use crate::{
	engine::{BitcoinClient, ReqPath, RpcResponse},
	IWallet, ReqBit,
};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
impl IWallet for ReqBit {
	async fn createwallet(&self, wallet_name: &str) -> Value {
		let client = BitcoinClient::new();
		let req_path = ReqPath::new(&client.config.bitcoin_node, "");

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

	async fn listreceivedbyaddress(
		&self,
		wallet_name: &str,
		minconf: Option<u32>,
		include_empty: Option<bool>,
	) -> Value {
		let client = BitcoinClient::new();

		let endpoint = "wallet/".to_string() + wallet_name;

		let req_path = ReqPath::new(&client.config.bitcoin_node, &endpoint);

		let minconf = minconf.unwrap_or(0);
		let include_empty = include_empty.unwrap_or(false);

		let rpc_response = client
			.send_request::<RpcResponse>(
				&req_path,
				"listreceivedbyaddress",
				vec![minconf.into(), include_empty.into()],
			)
			.await
			.unwrap();

		rpc_response.result
	}

	async fn listwallets(&self, wallet_name: &str) -> Value {
		let client = BitcoinClient::new();
		let endpoint = "wallet/".to_string() + wallet_name;
		let req_path = ReqPath::new(&client.config.bitcoin_node, &endpoint);

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "listwallets", vec![])
			.await
			.unwrap();

		rpc_response.result
	}

	async fn sendtoaddress(
		&self,
		wallet_name: &str,
		address: &str,
		amount: f64,
		comment: Option<String>,
		comment_to: Option<String>,
		subtract_fee_from_amount: Option<bool>,
	) -> Value {
		let client = BitcoinClient::new();
		let endpoint = "wallet/".to_string() + wallet_name;
		let req_path = ReqPath::new(&client.config.bitcoin_node, &endpoint);

		let mut params = vec![address.into(), amount.into()];

		if let Some(comment) = comment {
			params.push(comment.into());
			if let Some(comment_to) = comment_to {
				params.push(comment_to.into());
				if let Some(subtract_fee) = subtract_fee_from_amount {
					params.push(subtract_fee.into());
				}
			}
		}

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "sendtoaddress", params)
			.await
			.unwrap();

		rpc_response.result
	}

	async fn generatetoaddress(&self, nblocks: u32, address: &str, maxtries: Option<u32>) -> Value {
		let client = BitcoinClient::new();
		let req_path = ReqPath::new(&client.config.bitcoin_node, "");

		let mut params = vec![nblocks.into(), address.into()];

		if let Some(max_tries) = maxtries {
			params.push(max_tries.into());
		}

		println!("URL: {}", req_path.to_string());
		println!("Request body: {}", serde_json::to_string_pretty(&params).unwrap());

		let rpc_response =
			client.send_request::<RpcResponse>(&req_path, "generatetoaddress", params).await;

		rpc_response.unwrap().result
	}

	async fn gettransaction(
		&self,
		wallet_name: &str,
		txid: &str,
		include_watchonly: Option<bool>,
	) -> Value {
		let client = BitcoinClient::new();
		let req_path =
			ReqPath::new(&client.config.bitcoin_node, &format!("wallet/{}", wallet_name));

		let mut params = vec![txid.into()];

		if let Some(include_watch) = include_watchonly {
			params.push(include_watch.into());
		}

		println!("URL: {}", req_path.to_string());
		println!("Request body: {}", serde_json::to_string_pretty(&params).unwrap());

		let rpc_response = client
			.send_request::<RpcResponse>(&req_path, "gettransaction", params)
			.await
			.unwrap();

		rpc_response.result
	}
}
