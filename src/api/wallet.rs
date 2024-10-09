use crate::{
	engine::{BitcoinClient, RpcResponse},
	AddressType, IWallet, ReqBit,
};
use async_trait::async_trait;
use serde_json::{json, Value};

#[async_trait]
impl IWallet for ReqBit {
	async fn loadwallet(&self, wallet_name: &str, load_on_startup: Option<bool>) -> Value {
		let client = BitcoinClient::new();

		let mut params = vec![wallet_name.into()];
		if let Some(load_on_startup) = load_on_startup {
			params.push(json!({"load_on_startup": load_on_startup}));
		}

		let rpc_response =
			client.send_request::<RpcResponse>(None, "loadwallet", params).await.unwrap();

		rpc_response.result
	}
	async fn createwallet(&self, wallet_name: &str) -> Value {
		let client = BitcoinClient::new();

		let rpc_response = client
			.send_request::<RpcResponse>(None, "createwallet", vec![wallet_name.into()])
			.await
			.unwrap();

		rpc_response.result
	}

	async fn getwalletinfo(&self, wallet_name: &str) -> Value {
		let client = BitcoinClient::new();
		let endpoint = "wallet/".to_string() + wallet_name;

		let rpc_response = client
			.send_request::<RpcResponse>(Some(&endpoint), "getwalletinfo", vec![])
			.await
			.unwrap();

		rpc_response.result
	}

	async fn getbalance(&self, wallet_name: &str) -> Value {
		let client = BitcoinClient::new();
		let endpoint = "wallet/".to_string() + wallet_name;

		let rpc_response = client
			.send_request::<RpcResponse>(Some(&endpoint), "getbalance", vec![])
			.await
			.unwrap();

		rpc_response.result
	}

	async fn getnewaddress(
		&self,
		wallet_name: &str,
		label: Option<String>,
		address_type: Option<AddressType>,
	) -> Value {
		let client = BitcoinClient::new();

		let endpoint = "wallet/".to_string() + wallet_name;
		let label = label.unwrap_or("".into());
		let address_type = address_type.unwrap_or(AddressType::Legacy);

		let rpc_response = client
			.send_request::<RpcResponse>(
				Some(&endpoint),
				"getnewaddress",
				vec![label.into(), address_type.to_string().into()],
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
		let minconf = minconf.unwrap_or(0);
		let include_empty = include_empty.unwrap_or(false);

		let rpc_response = client
			.send_request::<RpcResponse>(
				Some(&endpoint),
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

		let rpc_response = client
			.send_request::<RpcResponse>(Some(&endpoint), "listwallets", vec![])
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
			.send_request::<RpcResponse>(Some(&endpoint), "sendtoaddress", params)
			.await
			.unwrap();

		rpc_response.result
	}

	async fn generatetoaddress(&self, nblocks: u32, address: &str, maxtries: Option<u32>) -> Value {
		let client = BitcoinClient::new();

		let mut params = vec![nblocks.into(), address.into()];

		if let Some(max_tries) = maxtries {
			params.push(max_tries.into());
		}

		println!("Request body: {}", serde_json::to_string_pretty(&params).unwrap());

		let rpc_response =
			client.send_request::<RpcResponse>(None, "generatetoaddress", params).await;

		rpc_response.unwrap().result
	}

	async fn gettransaction(
		&self,
		wallet_name: &str,
		txid: &str,
		include_watchonly: Option<bool>,
	) -> Value {
		let client = BitcoinClient::new();
		let endpoint = "wallet/".to_string() + wallet_name;

		let mut params = vec![txid.into()];

		if let Some(include_watch) = include_watchonly {
			params.push(include_watch.into());
		}

		println!("Request body: {}", serde_json::to_string_pretty(&params).unwrap());

		let rpc_response = client
			.send_request::<RpcResponse>(Some(&endpoint), "gettransaction", params)
			.await
			.unwrap();

		rpc_response.result
	}

	async fn setlabel(&self, wallet_name: &str, address: &str, label: &str) -> Value {
		let client = BitcoinClient::new();
		let endpoint = "wallet/".to_string() + wallet_name;

		let params = vec![address.into(), label.into()];

		println!("Request body: {}", serde_json::to_string_pretty(&params).unwrap());

		let rpc_response = client
			.send_request::<RpcResponse>(Some(&endpoint), "setlabel", params)
			.await
			.unwrap();

		rpc_response.result
	}

	async fn listunspent(
		&self,
		wallet_name: &str,
		minconf: Option<u32>,
		maxconf: Option<u32>,
		addresses: Option<Vec<String>>,
		include_unsafe: Option<bool>,
		query_options: Option<Value>,
	) -> Value {
		let client = BitcoinClient::new();
		let endpoint = "wallet/".to_string() + wallet_name;

		let mut params = Vec::new();
		params.push(minconf.unwrap_or(1).into());
		params.push(maxconf.unwrap_or(9999999).into());
		params.push(addresses.map_or(json!([]), |addrs| json!(addrs)));
		params.push(include_unsafe.unwrap_or(true).into());
		if let Some(options) = query_options {
			params.push(options);
		}

		println!("Request body: {}", serde_json::to_string_pretty(&params).unwrap());

		let rpc_response = client
			.send_request::<RpcResponse>(Some(&endpoint), "listunspent", params)
			.await
			.unwrap();

		rpc_response.result
	}

	async fn signrawtransactionwithwallet(
		&self,
		wallet_name: &str,
		hexstring: &str,
		prevtxs: Option<Vec<Value>>,
		sighash_type: Option<&str>,
	) -> Value {
		let client = BitcoinClient::new();
		let endpoint = "wallet/".to_string() + wallet_name;

		let mut params = vec![hexstring.into()];
		if let Some(prev) = prevtxs {
			params.push(json!(prev));
		}
		if let Some(sighash) = sighash_type {
			params.push(sighash.into());
		}

		println!("Request body: {}", serde_json::to_string_pretty(&params).unwrap());

		let rpc_response = client
			.send_request::<RpcResponse>(Some(&endpoint), "signrawtransactionwithwallet", params)
			.await
			.unwrap();

		rpc_response.result
	}
}
