use base64::{self, engine::general_purpose, Engine};
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::{env, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct ReqbitConfig {
	pub bitcoin_node: String,
	pub rpc_user: String,
	pub rpc_password: String,
}

impl ReqbitConfig {
	pub fn new() -> Result<Self, ConfigError> {
		let config_path = Self::get_config_path();

		let config = Config::builder()
			.add_source(
				File::with_name(config_path.to_str().unwrap()).format(config::FileFormat::Toml),
			)
			.build()?;

		config.try_deserialize()
	}

	pub fn get_auth(&self) -> String {
		let username = &self.rpc_user;
		let password = &self.rpc_password;
		let auth = general_purpose::STANDARD.encode(format!("{}:{}", username, password));

		auth
	}

	fn get_config_path() -> PathBuf {
		env::var("REQBIT_CONFIG").map(PathBuf::from).unwrap_or_else(|_| {
			let home = env::var("HOME").expect("HOME environment variable not set");
			PathBuf::from(home).join(".bitcoin").join("reqbit.toml")
		})
	}
}
