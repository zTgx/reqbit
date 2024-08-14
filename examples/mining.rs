pub use reqbit::{decorator, IMining, ReqBit};

#[tokio::main]
async fn main() {
	let reqbit = ReqBit;

	// let info = reqbit.get_block_template("segwit").await;
	// let info = reqbit.get_mining_info().await;
	let info = reqbit.getnetworkhashps(None, None).await;

	decorator::breautify(&info);
}
