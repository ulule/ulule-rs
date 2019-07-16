# Ulule

Rust API bindings for the Ulule v1 HTTP API.
This library rely on rust Futures to allow asynchronous usage.

[Ulule API documentation](https://developers.ulule.com/)

## Usage

Put this in `Cargo.toml`:

```toml
[dependencies]
ulule = "0.0.0"
```

and this in the crate root:

```rust
extern crate ulule;
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
let client = ulule::client::Client::new();
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
        search::projects(&client, Some(p))
})).unwrap();
```
