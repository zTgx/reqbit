use std::thread;
use zmq;

// This example demonstrates a ZeroMQ subscriber that listens for Bitcoin transactions.
// It connects to a local Bitcoin node running on port 28332 and prints received messages.
// The subscriber runs in a separate thread, allowing the main thread to perform other tasks.
// https://github.com/bitcoin/bitcoin/blob/master/doc/zmq.md
fn main() -> Result<(), Box<dyn std::error::Error>> {
	let context = zmq::Context::new();
	let subscriber = context.socket(zmq::SUB)?;
	subscriber.connect("tcp://127.0.0.1:28332")?;
	subscriber.set_subscribe(b"")?;

	thread::spawn(move || loop {
		match subscriber.recv_multipart(0) {
			Ok(message) => println!("Received transaction: {:?}", message),
			Err(e) => eprintln!("Error receiving message: {:?}", e),
		}
	});

	// Main thread can continue with other tasks
	println!("Subscription running in background. Press Ctrl+C to exit.");
	loop {
		// Keep the main thread alive
		std::thread::sleep(std::time::Duration::from_secs(1));
	}
}
