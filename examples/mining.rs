pub use reqbit::{
	api::{BitcoinCLI, IMining},
	decorator,
};

#[tokio::main]
async fn main() {
	let cli = BitcoinCLI;

	// let info = cli.get_block_template("segwit").await;
	// let info = cli.get_mining_info().await;
	let info = cli.getnetworkhashps(None, None).await;

	decorator::breautify(&info);
}
