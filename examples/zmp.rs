use std::thread;
use zmq;

// This example demonstrates a ZeroMQ subscriber that listens for Bitcoin transactions.
// It connects to a local Bitcoin node running on port 28332 and prints received messages.
// The subscriber runs in a separate thread, allowing the main thread to perform other tasks.
// https://github.com/bitcoin/bitcoin/blob/master/doc/zmq.md

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let context = zmq::Context::new();
	let subscriber = context.socket(zmq::SUB)?;

	println!("Connecting to ZMQ...");
	subscriber.connect("tcp://127.0.0.1:28332")?;
	println!("Connected to ZMQ");

	println!("Setting up subscriptions...");
	subscriber.set_subscribe(b"hashtx")?;
	subscriber.set_subscribe(b"rawtx")?;
	println!("Subscriptions set up");

	thread::spawn(move || {
		println!("Listening for messages...");
		loop {
			match subscriber.recv_multipart(0) {
				Ok(message) => println!("Received message: {:?}", message),
				Err(e) => eprintln!("Error receiving message: {:?}", e),
			}
		}
	});

	println!("Subscription running in background. Press Ctrl+C to exit.");
	loop {
		std::thread::sleep(std::time::Duration::from_secs(1));
	}
}
