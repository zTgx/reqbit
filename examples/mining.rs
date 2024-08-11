pub use reqbit::{
	api::{BitcoinCLI, IMining},
	decorator,
};

#[tokio::main]
async fn main() {
	let cli = BitcoinCLI;
	let info = cli.get_mining_info().await;

	decorator::breautify(&info);
}
