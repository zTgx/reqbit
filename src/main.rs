mod api;
mod decorator;
mod engine;
use api::getmininginfo::get_mining_info;

#[tokio::main]
async fn main() {
	get_mining_info().await;
}
