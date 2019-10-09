# ulule-client

[![ulule on crates.io](https://img.shields.io/crates/v/ulule-client.svg)](https://crates.io/crates/ulule-client)
[![ulule-rust on docs.rs](https://docs.rs/ulule-client/badge.svg)](https://docs.rs/ulule-client)

Rust API bindings for the Ulule v1 HTTP API.
This library rely on rust Futures to allow asynchronous usage.

[Ulule API documentation](https://developers.ulule.com/)

## Usage

Put this in `Cargo.toml`:

```toml
[dependencies]
ulule = "1.0.0"
ulule_client = "0.0.3"
```

and this in the crate root:

```rust
extern crate ulule;
extern crate ulule_client;
```

## Test

```
cargo test
```


## Examples

Run file from [examples](./examples) with:

```
cargo run --example <example> -- <example flags> <example args>
```

## Getting Started

To get started, create a client:

```rust
let client = ulule_client::Client::new();
```

Search for the last three project created matching the term `beer`
with their owner:

```rust
let p = search::Params::new()
        .limit(3)
        .with_term("beer")
        .with_extra_fields(vec!["owner".to_string()]);

// inside an actor system like actix_rt
let projects: search::Projects = actix_rt::System::new("test").block_on(lazy(|| {
    ulule_client::search_projects(&client, Some(p))
})).unwrap();
```
