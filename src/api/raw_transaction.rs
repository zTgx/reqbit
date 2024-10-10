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

	/**
	* decoderawtransaction <raw_transaction>
	* raw_transaction: a serialized transaction in hexadecimal notation.
	*
	* {
		   "txid": "466200308696215bbc949d5141a49a4138ecdfdfaa2a8029c1f9bcecd1f96177",
		   "hash": "f7cdbc7cf8b910d35cc69962e791138624e4eae7901010a6da4c02e7d238cdac",
		   "version": 1,
		   "size": 194,
		   "vsize": 143,
		   "weight": 569,
		   "locktime": 0,
		   "vin": [
			   {
				   "txid": "4ac541802679866935a19d4f40728bb89204d0cac90d85f3a51a19...aeb",
				   "vout": 1,
				   "scriptSig": {
					   "asm": "",
					   "hex": ""
				   },
				   "txinwitness": [
					   "cf5efe2d8ef13ed0af21d4f4cb82422d6252d70324f6f4576b727b7d918e5...301"
				   ],
				   "sequence": 4294967295
			   }
		   ],
		   "vout": [
			   {
				   "value": 0.0002,
				   "n": 0,
				   "scriptPubKey": {
					   "asm": "1 3b41daba4c9ace578369740f15e5ec880c28279ee7f51b07dca...068",
					   "desc": "rawtr(3b41daba4c9ace578369740f15e5ec880c28279ee7f51b...6ev",
					   "hex": "51203b41daba4c9ace578369740f15e5ec880c28279ee7f51b07d...068",
					   "address": "bc1p8dqa4wjvnt890qmfws83te0v3qxzsfu7ul63kp7u56w8q...5qn",
					   "type": "witness_v1_taproot"
				   }
			   },
			   {
				   "value": 0.00075,
				   "n": 1,
				   "scriptPubKey": {
					   "asm": "0 7752c165ea7be772b2c0acb7f4d6047ae6f4768e",
					   "desc": "addr(bc1qwafvze0200nh9vkq4jmlf4sy0tn0ga5w0zpkpg)#qq404gts",
					   "hex": "00147752c165ea7be772b2c0acb7f4d6047ae6f4768e",
					   "address": "bc1qwafvze0200nh9vkq4jmlf4sy0tn0ga5w0zpkpg",
					   "type": "witness_v0_keyhash"
				   }
			   }
		   ]
	   }
	*/
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
