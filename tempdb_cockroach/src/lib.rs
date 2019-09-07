//! Temporary [CockroachDB](https://www.cockroachlabs.com) databases for unit
//! testing.
//!
//! **Note:** The `cockroach` binary must be installed on your path.
//! Download it [here](https://www.cockroachlabs.com/docs/stable/install-cockroachdb.html).
//!
//! # Example
//!
//! ```rust
//! extern crate tempdb_cockroach;
//!
//! use tempdb_cockroach::TempCockroach;
//!
//! #[test]
//! fn test() {
//!     let db = TempCockroach::new().expect("Failed to create DB");
//!     println!("Connection string: {}", db.url());
//!
//!     // Cockroach process and data are cleaned up when db goes out of scope.
//! }
//! ```

extern crate postgres;
extern crate tempfile;

use std::error::Error;
use std::process::Child;
use std::process::Command;
use std::time::Duration;

use postgres::Connection;
use postgres::TlsMode;
use tempfile::TempDir;

/// A temporary CockroachDB instance.
///
/// Spawns a single-node CockroachDB instance in a subprocess, storing all data
/// in a temporary directory. Both the subprocess and temporary directory are
/// cleaned up when the `TempCockroach` object goes out of scope.
pub struct TempCockroach {
    /// Temporary directory where cockroach stores all its data.
    #[allow(dead_code)]
    tempdir: TempDir,

    /// Cockroach subprocess.
    process: Child,

    /// Database connection string.
    url: String,
}

impl TempCockroach {
    /// Create a new temporary CockroachDB instance.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Create the store directory.
        let tempdir = TempDir::new()?;

        // Spawn the cockroach subprocess.
        let url_file = tempdir.path().join("url");

        let process = Command::new("cockroach")
            .arg("start")
            .arg("--insecure")
            .arg("--listen-addr=localhost")
            .arg("--port=0")
            .arg("--http-port=0")
            .arg(format!("--listening-url-file={}", url_file.display()))
            .arg(format!("--store={}", tempdir.path().display()))
            .spawn()
            .map_err(|e| format!("Failed to start cockroach: {}. Did you remember to install it? See https://www.cockroachlabs.com/docs/stable/install-cockroachdb.html for installation instructions.", e.description()))?;

        // Wait for the URL file to be written.
        let mut url: String;
        loop {
            if url_file.exists() {
                match std::fs::read_to_string(url_file.clone()) {
                    Ok(s) => {
                        // Cockroach doesn't write the URL file atomically, so
                        // we look for the trailing newline to tell when it's
                        // complete.
                        if s.contains("\n") {
                            url = s.trim().to_string();
                            break;
                        }
                    }
                    Err(_) => {}
                };
            }
            std::thread::sleep(Duration::from_millis(10));
        }

        // Create test database.
        let conn = Connection::connect(url.clone(), TlsMode::None)?;
        conn.execute("CREATE DATABASE test", &[])?;
        url = url.replace("?sslmode=disable", "/test?sslmode=disable");

        Result::Ok(TempCockroach {
            tempdir: tempdir,
            process: process,
            url: url,
        })
    }

    /// Get the database connection string.
    pub fn url(&self) -> &String {
        &self.url
    }
}

impl Drop for TempCockroach {
    /// Called when the object goes out of scope to stop the database.
    fn drop(&mut self) {
        self.process.kill().expect("failed to kill cockroach");
        self.process
            .wait()
            .expect("failed to wait for cockroach to exit");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
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
}
