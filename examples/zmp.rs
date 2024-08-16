// This example demonstrates a ZeroMQ subscriber that listens for Bitcoin transactions.
// It connects to a local Bitcoin node running on port 28332 and prints received messages.
// The subscriber runs in a separate thread, allowing the main thread to perform other tasks.
// https://github.com/bitcoin/bitcoin/blob/master/doc/zmq.md

// bitcoincli generatetoaddress 1 $(bitcoincli getnewaddress)
use hex;
use reqbit::{
	api::{self, Block},
	IRawTransaction,
};
use zmq;

fn parse_block(raw_block: &[u8]) -> Result<Block, Box<dyn std::error::Error>> {
	let decoded_block = api::Block::try_from(raw_block)?;
	Ok(decoded_block)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let context = zmq::Context::new();
	let subscriber = context.socket(zmq::SUB)?;

	println!("Connecting to ZMQ...");
	subscriber.connect("tcp://127.0.0.1:28332")?;
	println!("Connected to ZMQ");

	println!("Setting up subscriptions...");
	subscriber.set_subscribe(b"hashtx")?;
	subscriber.set_subscribe(b"rawtx")?;
	subscriber.set_subscribe(b"hashblock")?;
	subscriber.set_subscribe(b"rawblock")?;
	println!("Subscriptions set up");

	tokio::spawn(async move {
		println!("Listening for messages...");
		loop {
			match subscriber.recv_multipart(0) {
				Ok(message) => {
					if message.len() >= 3 {
						let topic = std::str::from_utf8(&message[0]).unwrap_or("Unknown");
						let body = &message[1];
						let sequence = u32::from_le_bytes([
							message[2][0],
							message[2][1],
							message[2][2],
							message[2][3],
						]);

						println!("Topic: {}", topic);
						println!("Sequence: {}", sequence);

						match topic {
							"hashtx" | "hashblock" => {
								let hash = hex::encode(body);
								println!("Hash: {}", hash);
							},
							"rawtx" => {
								// println!("Raw transaction (first 20 bytes): {:?}",
								// &body[..20.min(body.len())]);

								// Decode raw transaction
								let decoded_tx = reqbit::ReqBit
									.decoderawtransaction(&hex::encode(body), None)
									.await;
								{
									println!("Decoded transaction: ");
									println!(
										"{}",
										serde_json::to_string_pretty(&decoded_tx).unwrap()
									);
								}
							},
							"rawblock" => {
								// println!("Raw block (first 20 bytes): {:?}",
								// &body[..20.min(body.len())]); Decode raw block
								match parse_block(body) {
									Ok(block) => {
										println!("Decoded block:");
										println!(
											"{}",
											serde_json::to_string_pretty(&block).unwrap()
										);
									},
									Err(e) => eprintln!("Error decoding block: {:?}", e),
								}
							},
							_ => println!("Unknown topic"),
						}
					} else {
						println!("Received message with unexpected format");
					}
					println!("--------------------");
				},
				Err(e) => eprintln!("Error receiving message: {:?}", e),
			}
		}
	});

	println!("Subscription running in background. Press Ctrl+C to exit.");
	loop {
		std::thread::sleep(std::time::Duration::from_secs(1));
	}
}
