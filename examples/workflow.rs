use reqbit::api::{BitcoinCLI, IWallet};

#[tokio::main]
async fn main() {
	let cli = BitcoinCLI;

	// 1. Create wallet A
	let wallet_a = "wallet_a";
	let create_wallet_a = cli.createwallet(wallet_a).await;
	println!("1. Created wallet A: {}", create_wallet_a);

	// 2. Mine 100 bitcoins
	let mining_address = cli.getnewaddress(wallet_a, None, None).await;
	let mined_blocks = cli.generatetoaddress(100, mining_address.as_str().unwrap(), None).await;
	println!("2. Mined 100 bitcoins: {}", mined_blocks);

	// 3. Create wallet B
	let wallet_b = "wallet_b";
	let create_wallet_b = cli.createwallet(wallet_b).await;
	println!("3. Created wallet B: {}", create_wallet_b);

	// 4. Get new address for wallet B
	let addr_b = cli.getnewaddress(wallet_b, None, None).await;
	println!("4. New address for wallet B: {}", addr_b);

	// 5. Send 3 coins from wallet A to wallet B's address
	let tx_id = cli
		.sendtoaddress(wallet_a, addr_b.as_str().unwrap(), 3.0, None, None, Some(true))
		.await;
	println!("5. Sent 3 coins from A to B. Transaction ID: {}", tx_id);

	// 6. Get transaction info
	let tx_info = cli.gettransaction(wallet_a, tx_id.as_str().unwrap(), None).await;
	println!("6. Transaction info: {}", tx_info);

	// 7. Compare wallet A and B balances
	let balance_a = cli.getbalance(wallet_a).await;
	let balance_b = cli.getbalance(wallet_b).await;
	println!("7. Wallet A balance: {} BTC", balance_a);
	println!("   Wallet B balance: {} BTC", balance_b);

	// 8. Final wallet info
	let wallet_a_info = cli.getwalletinfo(wallet_a).await;
	let wallet_b_info = cli.getwalletinfo(wallet_b).await;
	println!("8. Wallet A info: {}", wallet_a_info);
	println!("   Wallet B info: {}", wallet_b_info);
}
