# Import in Cargo.toml

```
common-tendermint = { git = "https://github.com/scalarorg/common", branch = "dev", package = "common-tendermint", features = ["client", "v0_37"] }
```

# Run test

```
cargo test abci_echo --features=client,v0_37 -- --nocapture
```
