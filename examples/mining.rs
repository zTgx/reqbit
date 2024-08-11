
pub use reqbit::{decorator, api::{BitcoinCLI, IMining}};

#[tokio::main]
async fn main() {
	let cli = BitcoinCLI;
	let info = cli.get_mining_info().await;

	decorator::breautify(&info);
}
