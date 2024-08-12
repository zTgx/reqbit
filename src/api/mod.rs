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
