use reqbit::{IBlockchain, ReqBit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let reqbit = ReqBit;

	// Example block hash (replace with a valid block hash)
	let block_hash = "000000000000000000076c036ff5119e5a5a74df77abf64203473364509f7732";

	// Step 1: Get the block
	let block = reqbit.getblock(block_hash, Some(2)).await;

	// Step 2: Extract transaction data
	let transactions = block["tx"].as_array().ok_or("No transactions found")?;

	// Step 3: Calculate total block value
	let mut total_value = 0.0;

	for tx in transactions {
		if let Some(outputs) = tx["vout"].as_array() {
			for output in outputs {
				if let Some(value) = output["value"].as_f64() {
					total_value += value;
				}
			}
		}
	}

	println!("Block hash: {}", block_hash);
	println!("Total value of all outputs: {} BTC", total_value);

	Ok(())
}
