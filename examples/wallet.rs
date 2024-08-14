pub use reqbit::{decorator, IWallet, ReqBit};

#[tokio::main]
async fn main() {
	let reqbit = ReqBit;

	// let info = reqbit.createwallet("bob").await;
	// let info = reqbit.getwalletinfo("bob").await;
	// println!(">> list info: {:#?}", info);

	// let info = reqbit.getnewaddress("bob", None, None).await;
	// let info = reqbit.listreceivedbyaddress("bob", None, Some(true)).await;
	// let info = reqbit.listwallets("bob").await;
	// println!(">> list address: {:#?}", info);

	// let info = reqbit.sendtoaddress("bob", "bcrt1qkjask97dyu8wdc3lzp96ylsjhghea682np9u9a", 3.0,
	// None, None, Some(true)).await; println!(">> send to address: {:#?}", info);

	// let info = reqbit.generatetoaddress(1, "mp7esTo7hhSwFzi4HzUGZTBGizYH7yekEi", None).await;
	// println!(">> generate to address: {:#?}", info);

	let info = reqbit
		.gettransaction(
			"bob",
			"d9c926b512b9c3129341b22ce446671c768bf0957456b8786469cf3515956edd",
			None,
		)
		.await;

	decorator::breautify(&info);
}
