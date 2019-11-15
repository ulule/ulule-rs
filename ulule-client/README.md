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
use ulule::search;
use ulule_client::{search_projects, Client};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let p = search::Params::new()
        .limit(2)
        .with_term("beer")
        .with_extra_fields(vec![
            "owner".to_string(),
            "main_tag".to_string(),
            "main_image".to_string(),
        ]);

    let first_page = search_projects(&client, Some(p)).await.unwrap();
    println!("first page: {:?}", first_page);

    if !first_page.meta.has_next() {
        return;
    }

    let second_page = search_projects(&client, first_page.meta.next).await.unwrap();
    println!("second page: {:?}", second_page)
}
```
