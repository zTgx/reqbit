pub use reqbit::{
	api::{BitcoinCLI, IWallet},
	decorator,
};

#[tokio::main]
async fn main() {
	let cli = BitcoinCLI;

	// let info = cli.createwallet("bob").await;
	// let info = cli.getwalletinfo("bob").await;
	// let info = cli.getnewaddress("bob", None, None).await;
	// let info = cli.listreceivedbyaddress("bob", None, Some(true)).await;
	let info = cli.listwallets("bob").await;

	decorator::breautify(&info);
}
