use serde_json::Value;
use async_trait::async_trait;

pub mod mining;

pub struct BitcoinCLI;

#[async_trait]
pub trait IMining {
    async fn get_mining_info(&self) -> Value;
}