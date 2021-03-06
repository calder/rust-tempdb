# <a href="https://cockroachlabs.com"><img src="https://raw.githubusercontent.com/calder/rust-tempdb/master/tempdb_cockroach/doc/logo.png" width="24" height="24"></a> Rust TempDB - [CockroachDB](https://www.cockroachlabs.com)

[![Documentation](https://docs.rs/tempdb_cockroach/badge.svg)](https://docs.rs/tempdb_cockroach) [![Latest Version](https://img.shields.io/crates/v/tempdb_cockroach.svg)](https://crates.io/crates/tempdb_cockroach) [![Build Status](https://travis-ci.org/calder/rust-tempdb.svg?branch=master)](https://travis-ci.org/calder/rust-tempdb) [![Coverage Status](https://coveralls.io/repos/github/calder/rust-tempdb/badge.svg?branch=master)](https://coveralls.io/github/calder/rust-tempdb?branch=master)

Temporary [CockroachDB](https://www.cockroachlabs.com) databases for unit testing.

## Installation

Add the following to your `Cargo.toml`:
```toml
[dev-dependencies]
tempdb_cockroach = ""
```

Install `libpq-dev` (required by the [`postgres`](https://docs.rs/postgres/) crate):
```sh
sudo apt install libpq-dev
```

[Install cockroach](https://www.cockroachlabs.com/docs/stable/install-cockroachdb.html) if you haven't already:
```sh
wget -qO- https://binaries.cockroachdb.com/cockroach-latest.linux-amd64.tgz | tar -xvz
sudo cp cockroach-*.linux-amd64/cockroach /usr/local/bin
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
