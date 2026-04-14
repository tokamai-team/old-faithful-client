# yellowstone-faithful-client

Rust SDK for interacting with [Old Faithful](https://github.com/rpcpool/yellowstone-faithful) gRPC services, providing access to Solana historical data.

## Features

- Unary RPCs: `GetVersion`, `GetBlock`, `GetBlockTime`, `GetTransaction`
- Bidirectional streaming: `Get`
- Server streaming: `StreamBlocks`, `StreamTransactions`
- Optional `x-token` authentication via a tonic interceptor
- Typed domain models over the raw protobuf messages

## Installation

```toml
[dependencies]
yellowstone-faithful-client = "0.1"
```

## Quick start

```rust
use yellowstone_faithful_client::{connect_with_config, GrpcConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = GrpcConfig::new("https://your-old-faithful-endpoint".to_string());
    let mut client = connect_with_config(config).await?;
    let version = client.get_version().await?;
    println!("Old Faithful version: {}", version.version);
    Ok(())
}
```

## Examples

See the [`examples/`](examples/) directory for complete, runnable programs covering all supported RPCs. Run with:

```bash
cargo run --example get_version -- --endpoint <url> [--x-token <token>]
```

## Building

This crate compiles the Old Faithful `.proto` files at build time via `tonic-build`. `protoc` is provided by the `protobuf-src` build dependency, so no system-wide install is required.

```bash
cargo build
```

## License

MIT — see [LICENSE](LICENSE).
