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

## Streaming transactions for a single program across a slot range

A common backfill pattern: replay every transaction that touched a given program
between two slots. `StreamTransactions` does the heavy lifting server-side —
Old Faithful walks the CAR archives, applies the `account_include` filter, and
yields transactions as they are decoded. This is how `panoptes` backfills
per-program watchlists.

```rust
use futures::StreamExt;
use yellowstone_faithful_client::{connect_with_config, GrpcConfig, StreamTransactionsFilter};

// Pump.fun program — any pubkey the transaction references (signer, writable,
// readonly, or invoked program) counts as a match.
const PUMP_FUN: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = GrpcConfig::new("https://your-old-faithful-endpoint".to_string())
        .with_token("your-x-token".to_string());
    let mut client = connect_with_config(config).await?;

    let filter = StreamTransactionsFilter {
        vote: Some(false),
        failed: None,
        account_include: vec![PUMP_FUN.to_string()],
        account_exclude: vec![],
        account_required: vec![],
    };

    let mut stream = client
        .stream_transactions(307_000_000, Some(307_001_000), Some(filter))
        .await?;

    while let Some(tx) = stream.next().await.transpose()? {
        println!("slot={} index={:?} size={}B", tx.slot, tx.index, tx.transaction.transaction.len());
    }
    Ok(())
}
```

Tips:

- `end_slot = None` streams indefinitely into the live tip.
- `account_required` is an AND across all listed accounts; `account_include` is
  an OR. Use `account_required` when you need a transaction to touch *both* a
  program and a specific token account.
- For whole-block reconstruction (including rewards and block metadata), use
  `stream_blocks` with the same `account_include` filter instead.

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
