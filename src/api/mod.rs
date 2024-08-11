use async_trait::async_trait;
use serde_json::Value;

pub mod mining;

/// Represents a Bitcoin Command Line Interface (CLI)
///
/// This struct serves as a container for implementing various Bitcoin-related
/// operations, such as mining functions defined in the `IMining` trait.
pub struct BitcoinCLI;

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
}
