use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct ResponseData {
	message: String,
}

#[tokio::main]
async fn main() {
	// Define the mock endpoint
	let hello = warp::path("api")
		.and(warp::path("hello"))
		.map(|| warp::reply::json(&ResponseData { message: String::from("Hello, world!") }));

	// Start the server
	warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
