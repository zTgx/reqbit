pub use reqbit::{
	api::{BitcoinCLI, IWallet},
	decorator,
};

#[tokio::main]
async fn main() {
	let cli = BitcoinCLI;

	// let info = cli.createwallet("bob").await;
	// let info = cli.getwalletinfo("bob").await;
	// println!(">> list info: {:#?}", info);

	// let info = cli.getnewaddress("bob", None, None).await;
	// let info = cli.listreceivedbyaddress("bob", None, Some(true)).await;
	// let info = cli.listwallets("bob").await;
	// println!(">> list address: {:#?}", info);

	// let info = cli.sendtoaddress("bob", "bcrt1qkjask97dyu8wdc3lzp96ylsjhghea682np9u9a", 3.0,
	// None, None, Some(true)).await; println!(">> send to address: {:#?}", info);

	// let info = cli.generatetoaddress(1, "mp7esTo7hhSwFzi4HzUGZTBGizYH7yekEi", None).await;
	// println!(">> generate to address: {:#?}", info);

	let info = cli
		.gettransaction(
			"bob",
			"d9c926b512b9c3129341b22ce446671c768bf0957456b8786469cf3515956edd",
			None,
		)
		.await;

	decorator::breautify(&info);
}
