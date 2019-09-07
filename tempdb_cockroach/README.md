# <a href="https://cockroachlabs.com"><img src="https://github.com/calder/rust-tempdb/raw/master/tempdb_cockroach/doc/logo.svg" width="24" height="24"></a> Rust TempDB - [CockroachDB](https://www.cockroachlabs.com)

[![Documentation](https://docs.rs/tempdb_cockroach/badge.svg)](https://docs.rs/tempdb_cockroach) [![Latest Version](https://img.shields.io/crates/v/tempdb_cockroach.svg)](https://crates.io/crates/tempdb_cockroach)

Temporary [CockroachDB](https://www.cockroachlabs.com) databases for unit testing.

## Installation

Add the following to your `Cargo.toml`:
```toml
[dev-dependencies]
tempdb_cockroach = ""
```

[Install cockroach](https://www.cockroachlabs.com/docs/stable/install-cockroachdb.html) if you haven't already:
```sh
wget -qO- https://binaries.cockroachdb.com/cockroach-v19.1.4.linux-amd64.tgz | tar -xvz
cp -i cockroach-v19.1.4.linux-amd64/cockroach /usr/local/bin
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
