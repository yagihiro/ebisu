# ebisu

## Motivation

...

## Build Status 

[![Build Status](https://travis-ci.org/yagihiro/ebisu.svg?branch=master)](https://travis-ci.org/yagihiro/ebisu)

## Getting Started

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

## Enable ebisu log

```sh
% RUST_LOG=ebisu=debug cargo run
```
