# 💾 Rust TempDB

[![Build Status](https://travis-ci.org/calder/rust-tempdb.svg?branch=master)](https://travis-ci.org/calder/rust-tempdb) [![Coverage Status](https://coveralls.io/repos/github/calder/rust-tempdb/badge.svg?branch=master)](https://coveralls.io/github/calder/rust-tempdb?branch=master)

Temporary databases for unit testing, inspired by [`tempfile`](https://docs.rs/tempfile).

| Database | Crate | Documentation | Latest |
| --- | --- | --- | --- |
| <a href="https://cockroachlabs.com"><img src="https://raw.githubusercontent.com/calder/rust-tempdb/master/tempdb_cockroach/doc/logo.png" width="12" height="12"> CockroachDB</a> | [`tempdb_cockroach`](tempdb_cockroach) | [![Documentation](https://docs.rs/tempdb_cockroach/badge.svg)](https://docs.rs/tempdb_cockroach) | [![Latest Version](https://img.shields.io/crates/v/tempdb_cockroach.svg)](https://crates.io/crates/tempdb_cockroach) |

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

Feel free to submit pull requests for new databases or other improvements! Run `scripts/install-git-hooks` to install pre-commit test and formatting hooks.

This project follows the Rust community's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
