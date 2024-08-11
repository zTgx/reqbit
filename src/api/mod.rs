use async_trait::async_trait;
use serde_json::Value;

pub mod mining;

pub struct BitcoinCLI;

#[async_trait]
pub trait IMining {
	async fn get_mining_info(&self) -> Value;
}
