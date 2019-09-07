# <a href="https://cockroachlabs.com"><img src="doc/logo.png" width="24" height="24"></a> Rust TempDB - [CockroachDB](https://www.cockroachlabs.com)

[![Documentation](https://docs.rs/tempdb_cockroach/badge.svg)](https://docs.rs/tempdb_cockroach) [![Latest Version](https://img.shields.io/crates/v/tempdb_cockroach.svg)](https://crates.io/crates/tempdb_cockroach)

Temporary [CockroachDB](https://www.cockroachlabs.com) databases for unit testing.

## Installation

Add the following to your `Cargo.toml`:
```
[dev-dependencies]
tempdb_cockroach = ""
```

## Usage

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
