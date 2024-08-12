use async_trait::async_trait;
use serde_json::Value;

pub mod mining;
pub mod wallet;

/// Represents a Bitcoin Command Line Interface (CLI)
///
/// This struct serves as a container for implementing various Bitcoin-related
/// operations, such as mining functions defined in the `IMining` trait.
pub struct BitcoinCLI;

/// Trait for Bitcoin wallet-related operations
#[async_trait]
pub trait IWallet {
	/// Creates a new wallet with the given name
	///
	/// # Arguments
	///
	/// * `wallet_name` - A string slice that holds the name of the wallet to be created
	///
	/// # Returns
	///
	/// Returns a `Value` containing information about the created wallet
	async fn createwallet(&self, wallet_name: &str) -> Value;

	/// Retrieves information about a specified wallet
	///
	/// # Arguments
	///
	/// * `wallet_name` - A string slice that holds the name of the wallet to query
	///
	/// # Returns
	///
	/// Returns a `Value` containing detailed information about the specified wallet
	async fn getwalletinfo(&self, wallet_name: &str) -> Value;

	/// Retrieves the balance of a specified wallet
	///
	/// # Arguments
	///
	/// * `wallet_name` - A string slice that holds the name of the wallet to query
	///
	/// # Returns
	///
	/// Returns a `Value` containing the balance of the specified wallet
	async fn getbalance(&self, wallet_name: &str) -> Value;

	/// Generates a new Bitcoin address for receiving payments
	///
	/// # Arguments
	///
	/// * `label` - An optional string to assign a label to the address
	/// * `address_type` - An optional string to specify the type of address to generate (e.g.,
	///   "legacy", "p2sh-segwit", "bech32")
	///
	/// # Returns
	///
	/// Returns a `Value` containing the newly generated Bitcoin address
	async fn getnewaddress(
		&self,
		wallet_name: &str,
		label: Option<String>,
		address_type: Option<String>,
	) -> Value;

	/// Lists received transactions by address for a specific wallet
	///
	/// # Arguments
	///
	/// * `wallet_name` - A string slice that holds the name of the wallet to query
	/// * `minconf` - An optional minimum number of confirmations for transactions to be included
	/// * `include_empty` - An optional boolean to include addresses that haven't received any
	///   payments
	///
	/// # Returns
	///
	/// Returns a `Value` containing a list of received transactions grouped by address
	async fn listreceivedbyaddress(
		&self,
		wallet_name: &str,
		minconf: Option<u32>,
		include_empty: Option<bool>,
	) -> Value;

	/// Lists all available wallets
	///
	/// # Arguments
	///
	/// * `wallet_name` - A string slice that holds the name of the wallet (Note: This parameter
	///   might not be necessary for this function and could be removed in a future update)
	///
	/// # Returns
	///
	/// Returns a `Value` containing a list of all available wallets
	async fn listwallets(&self, wallet_name: &str) -> Value;

	/// Sends an amount of bitcoins to a specified address
	///
	/// # Arguments
	///
	/// * `wallet_name` - A string slice that holds the name of the wallet to send from
	/// * `address` - The Bitcoin address to send to
	/// * `amount` - The amount of bitcoins to send
	/// * `comment` - An optional comment to store with the transaction
	/// * `comment_to` - An optional comment to store the name of the recipient
	/// * `subtract_fee_from_amount` - An optional boolean to subtract the fee from the amount being
	///   sent
	///
	/// # Returns
	///
	/// Returns a `Value` containing the transaction ID of the sent transaction
	async fn sendtoaddress(
		&self,
		wallet_name: &str,
		address: &str,
		amount: f64,
		comment: Option<String>,
		comment_to: Option<String>,
		subtract_fee_from_amount: Option<bool>,
	) -> Value;

	/// Generates blocks and mines them to a specified address
	///
	/// # Arguments
	///
	/// * `nblocks` - The number of blocks to generate
	/// * `address` - The address to send the newly generated bitcoin to
	/// * `maxtries` - An optional maximum number of iterations to try (default: 1000000)
	///
	/// # Returns
	///
	/// Returns a `Value` containing a list of block hashes of the generated blocks
	async fn generatetoaddress(&self, nblocks: u32, address: &str, maxtries: Option<u32>) -> Value;

	/// Retrieves detailed information about a specific transaction
	///
	/// # Arguments
	///
	/// * `wallet_name` - A string slice that holds the name of the wallet
	/// * `txid` - A string slice that holds the transaction ID
	/// * `include_watchonly` - An optional boolean to include watch-only addresses (default: false)
	///
	/// # Returns
	///
	/// Returns a `Value` containing detailed information about the transaction
	async fn gettransaction(
		&self,
		wallet_name: &str,
		txid: &str,
		include_watchonly: Option<bool>,
	) -> Value;
}

/// Trait for Bitcoin mining-related operations
#[async_trait]
pub trait IMining {
	/// Retrieves a block template for mining
	///
	/// # Arguments
	///
	/// * `rules` - A string slice that holds the rules for block template generation
	///
	/// # Returns
	///
	/// Returns a `Value` containing the block template
	async fn get_block_template(&self, rules: &str) -> Value;

	/// Retrieves current mining-related information
	///
	/// # Returns
	///
	/// Returns a `Value` containing various mining statistics and information
	async fn get_mining_info(&self) -> Value;

	/// Retrieves the estimated network hashes per second
	///
	/// # Arguments
	///
	/// * `nblocks` - The number of blocks to look back (default: 120)
	/// * `height` - The block height to estimate at (default: -1 for current best tip)
	///
	/// # Returns
	///
	/// Returns a `Value` containing the estimated network hash rate in hashes per second
	async fn getnetworkhashps(&self, nblocks: Option<i32>, height: Option<u32>) -> Value;
}
