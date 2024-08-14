pub use reqbit::{
	api::{BitcoinCLI, IBlockchain},
	decorator,
};

#[tokio::main]
async fn main() {
	let cli = BitcoinCLI;
	let info = cli
		.getblock("600ffdab0fff1d7a7314e86197adc0149b67a5d79bd71dd88458710d3d5d0c71", None)
		.await;

	decorator::breautify(&info);
}
