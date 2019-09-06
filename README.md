# 💾 Rust TempDB

[![Documentation](https://docs.rs/tempdb/badge.svg)](https://docs.rs/tempdb) [![Latest Version](https://img.shields.io/crates/v/tempdb.svg)](https://crates.io/crates/tempdb) [![Build Status](https://api.travis-ci.org/calder/rust-tempdb.svg?branch=master)](https://travis-ci.org/calder/rust-tempdb) [![Coverage Stats](https://codecov.io/gh/calder/rust-tempdb/branch/master/graph/badge.svg)](https://codecov.io/gh/calder/rust-tempdb)

Temporary databases for unit testing against.

## Usage

```rust
extern crate postgres;
extern crate tempdb;

use postgres::Connection;
use postgres::TlsMode;
use tempdb::cockroach::TempCockroach;

#[test]
fn test() {
    let db = TempCockroach::new().expect("Failed to create DB");
    let conn =
        Connection::connect(db.url().as_str(), TlsMode::None).expect("Failed to connect to DB");

    conn.execute("CREATE TABLE users (name VARCHAR)", &[])
        .expect("Failed to create table");
    conn.execute("INSERT INTO users (name) VALUES ('Alice'), ('Bob')", &[])
        .expect("Failed to insert into table");
    let rows = conn
        .query("SELECT * FROM users", &[])
        .expect("Failed to read table");
    assert_eq!(2, rows.len());
    assert_eq!("Alice", rows.get(0).get::<&str, String>("name"));
    assert_eq!("Bob", rows.get(1).get::<&str, String>("name"));
}
```

## Contributing

Feel free to submit pull requests for new databases or anything else that's missing!

This project follows the Rust community's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
