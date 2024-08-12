pub use reqbit::{
	api::{BitcoinCLI, IWallet},
	decorator,
};

#[tokio::main]
async fn main() {
	let cli = BitcoinCLI;

	let info = cli.createwallet("bob").await;

	decorator::breautify(&info);
}
