# 💾 Rust TempDB

[![Documentation](https://docs.rs/tempdb/badge.svg)](https://docs.rs/tempdb) [![Latest Version](https://img.shields.io/crates/v/tempdb.svg)](https://crates.io/crates/tempdb) [![Build Status](https://api.travis-ci.org/calder/rust-tempdb.svg?branch=master)](https://travis-ci.org/calder/rust-tempdb) [![Coverage Stats](https://codecov.io/gh/calder/rust-tempdb/branch/master/graph/badge.svg)](https://codecov.io/gh/calder/rust-tempdb)

`tempdb` spins up temporary databases for unit testing against.

## Usage

```rust
extern crate tempdb;

use tempdb::cockroach::TempCockroach;

#[test]
fn test() {
    let db = TempCockroach::new().expect("failed to create DB");
    println!("{:?}", db.connection_string());
}
```

## Contributing

Feel free to submit pull requests for new databases or anything else that's missing!

This project follows the Rust community's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
