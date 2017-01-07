# ebisu

## Motivation

...

## Getting started

```toml
[dependencies]
ebisu = { git = "https://github.com/yagihiro/ebisu.git" }
```

```rust
extern crate ebisu;

fn main() {
    let url = "mysql://user:pass@127.0.0.1/?database=yourdbname";
    let mut db = ebisu::db::connect_with_url(url);
    
    // ...
}
```

## Log

```sh
% RUST_LOG=ebisu=debug cargo run
```
