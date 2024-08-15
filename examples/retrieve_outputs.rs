use reqbit::{IRawTransaction, ReqBit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let reqbit = ReqBit;

	// Example transaction ID
	let txid = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

	// Step 1: Get the raw transaction
	let raw_tx = reqbit.getrawtransaction(txid, Some(false), None).await;

	// Step 2: Decode the raw transaction
	let decoded_tx = reqbit.decoderawtransaction(raw_tx.as_str().unwrap(), None).await;

	// Step 3: Retrieve and print the outputs
	if let Some(outputs) = decoded_tx["vout"].as_array() {
		println!("Transaction outputs:");
		for (index, output) in outputs.iter().enumerate() {
			let amount = output["value"].as_f64().unwrap_or(0.0);
			let address = output["scriptPubKey"]["address"].as_str().unwrap_or("Unknown");
			println!("Output {}: {} BTC to {}", index, amount, address);
		}
	} else {
		println!("No outputs found in the transaction.");
	}

	Ok(())
}
