use reqbit::{IRawTransaction, IWallet, ReqBit};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let reqbit = ReqBit;

	// pre cond:
	let wallet_name = "bob";
	let utxos = reqbit.listunspent(wallet_name, None, None, None, None, None).await;
	let first_utxo = utxos
		.as_array()
		.and_then(|arr| arr.first())
		.ok_or("No unspent transactions found")?;

	// Step 1: Create a raw transaction
	let inputs = vec![json!({
		"txid": first_utxo["txid"],
		"vout": first_utxo["vout"]
	})];
	let outputs = vec![json!({
		"bcrt1qyouraddresshere": 0.001
	})];
	let raw_tx = reqbit.createrawtransaction(inputs, outputs, None).await;
	println!("Raw transaction: {}", raw_tx);

	// Step 2: Sign the raw transaction
	let privkeys = vec!["your_private_key_here".to_string()];
	let signed_tx =
		reqbit.signrawtransactionwithkey(raw_tx.as_str().unwrap(), privkeys, None).await;
	println!("Signed transaction: {}", signed_tx);

	// Step 3: Send the raw transaction
	let tx_hash = reqbit.sendrawtransaction(signed_tx["hex"].as_str().unwrap()).await;
	println!("Transaction hash: {}", tx_hash);

	// Step 4: Get transaction details
	let wallet_name = ""; // Replace with your wallet name
	let transaction_details =
		reqbit.gettransaction(wallet_name, tx_hash.as_str().unwrap(), None).await;
	println!("Transaction details: {}", transaction_details);

	Ok(())
}
