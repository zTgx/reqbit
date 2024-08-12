extern crate async_trait;

pub mod api;
pub mod decorator;
mod engine;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
	pub name: String,
	// Add other wallet-specific fields here
}

impl Wallet {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_string(),
			// Initialize other fields here
		}
	}

	pub fn load(name: &str) -> Self {
		// In a real implementation, you might load wallet data from storage
		Self::new(name)
	}
}
