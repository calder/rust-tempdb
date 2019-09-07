# 💾 Rust TempDB

[![Build Status](https://api.travis-ci.org/calder/rust-tempdb.svg?branch=master)](https://travis-ci.org/calder/rust-tempdb) [![Coverage Stats](https://codecov.io/gh/calder/rust-tempdb/branch/master/graph/badge.svg)](https://codecov.io/gh/calder/rust-tempdb)

Temporary databases for unit testing, inspired by [`tempfile`](https://docs.rs/tempfile).
* [`tempdb_cockroach`](tempdb_cockroach) - Single-node [CockroachDB](https://www.cockroachlabs.com) instances.

## Example usage

```rust
extern crate tempdb_cockroach;

use tempdb_cockroach::TempCockroach;

#[test]
fn test() {
    let db = TempCockroach::new().expect("Failed to create DB");
    println!("Connection string: {}", db.url());

    // Cockroach process and data are cleaned up when db goes out of scope.
}
```

## Contributing

Feel free to submit pull requests for new databases or anything else that's missing!

This project follows the Rust community's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
