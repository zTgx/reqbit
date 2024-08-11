pub use reqbit::{
	api::{BitcoinCLI, IMining},
	decorator,
};

#[tokio::main]
async fn main() {
	let cli = BitcoinCLI;
	let template = cli.get_block_template("segwit").await;
	// let info = cli.get_mining_info().await;

	decorator::breautify(&template);
}
