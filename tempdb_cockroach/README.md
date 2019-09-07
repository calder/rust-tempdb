# ![💾](doc/logo.png) Rust TempDB - [CockroachDB](https://www.cockroachlabs.com)

[![Documentation](https://docs.rs/tempdb_cockroach/badge.svg)](https://docs.rs/tempdb_cockroach) [![Latest Version](https://img.shields.io/crates/v/tempdb_cockroach.svg)](https://crates.io/crates/tempdb_cockroach) [![Build Status](https://api.travis-ci.org/calder/rust-tempdb.svg?branch=master)](https://travis-ci.org/calder/rust-tempdb) [![Coverage Stats](https://codecov.io/gh/calder/rust-tempdb/branch/master/graph/badge.svg)](https://codecov.io/gh/calder/rust-tempdb)

Temporary [CockroachDB](https://www.cockroachlabs.com) databases for unit testing.

## Usage

```rust
extern crate postgres;
extern crate tempdb_cockroach;

use postgres::Connection;
use postgres::TlsMode;
use tempdb_cockroach::TempCockroach;

fn create_users_table(conn: &Connection) {
    conn.execute("CREATE TABLE users (name VARCHAR)", &[])
        .expect("Failed to create table");
}

fn create_users(conn: &Connection) {
    conn.execute("INSERT INTO users (name) VALUES ('Alice'), ('Bob')", &[])
        .expect("Failed to insert into table");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_users() {
        let db = TempCockroach::new().expect("Failed to create DB");
        let conn =
            Connection::connect(db.url().as_str(), TlsMode::None).expect("Failed to connect to DB");

        create_users_table(&conn);
        create_users(&conn);

        let rows = conn
            .query("SELECT * FROM users", &[])
            .expect("Failed to read table");
        assert_eq!(2, rows.len());
        assert_eq!("Alice", rows.get(0).get::<&str, String>("name"));
        assert_eq!("Bob", rows.get(1).get::<&str, String>("name"));
    }
}
```

## Contributing

Feel free to submit pull requests for new databases or anything else that's missing!

This project follows the Rust community's [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).
