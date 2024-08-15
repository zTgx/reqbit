# ReqBit

[![Crates.io](https://img.shields.io/crates/v/reqbit.svg)](https://crates.io/crates/reqbit)

ReqBit is a Rust library that provides a robust and user-friendly interface for interacting with Bitcoin Core's RPC API. It simplifies Bitcoin-related operations, making it easier for developers to build and maintain Bitcoin applications.

## Key Features

- **Wallet Management**: Easily load, create, and query Bitcoin wallets
- **Mining Operations**: Retrieve block templates, mining information, and network hash rates
- **Blockchain Interaction**: Access detailed block information and blockchain state
- **Transaction Handling**: Create, sign, and broadcast raw transactions

## Why ReqBit?

- **Simplified API**: Abstracts complex Bitcoin Core RPC calls into intuitive Rust functions
- **Asynchronous Support**: Built with async/await for efficient concurrent operations
- **Type-Safe**: Leverages Rust's strong type system to prevent runtime errors
- **Extensible**: Modular design allows for easy addition of new features

## Getting Started

To use ReqBit in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
reqbit = "0.1.0"
```

Then, you can start using ReqBit in your Rust code:
```rust
use reqbit::ReqBit;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reqbit = ReqBit;
	let info = reqbit.getnetworkhashps(None, None).await;

    Ok(())
}
```

## Configuration

ReqBit requires a running Bitcoin Core node. Make sure to configure your `bitcoin.conf` file with the following settings:

```
server=1
rpcuser=your_username
rpcpassword=your_password
rpcallowip=127.0.0.1
```


## Examples

Check out the `examples/` directory for more detailed usage examples, including:

- Wallet creation and management
- Sending transactions
- Mining operations
- Blockchain queries

## Documentation

For full API documentation, run:

```
cargo doc --open
```


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
