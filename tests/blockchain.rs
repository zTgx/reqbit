use reqwest::{Client, StatusCode};

#[tokio::test]
async fn test_hello_endpoint() {
	let client = Client::new();
	let resp = client
		.get("http://localhost:3030/api/hello")
		.send()
		.await
		.expect("Failed to send request");

	assert_eq!(resp.status(), StatusCode::OK);
	// println!("resp: {:?}", resp.json().await);

	// let body = resp.json::<ResponseData>().expect("Failed to parse JSON");
	// assert_eq!(body.message, "Hello, world!");
}
