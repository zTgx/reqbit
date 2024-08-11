use crate::{decorator, engine::{BitcoinClient, RpcResponse}};

pub async fn get_mining_info() {
	let client = BitcoinClient::new();

	let rpc_response = client.send_request::<RpcResponse>("getmininginfo", vec![]).await.unwrap();

    decorator::breautify(&rpc_response.result);
}
