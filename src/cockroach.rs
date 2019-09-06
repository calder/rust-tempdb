extern crate postgres;
extern crate tempdir;

use std::error::Error;
use std::process::Child;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use postgres::Connection;
use postgres::TlsMode;
use tempdir::TempDir;

pub struct TempCockroach {
    #[allow(dead_code)]
    tempdir: TempDir,

    process: Child,

    url: String,
}

impl TempCockroach {
    pub fn new() -> Result<Self, Box<Error>> {
        let tempdir = TempDir::new("cockroach-data")?;

        let url_file = tempdir.path().join("url");
        let process = Command::new("cockroach")
            .arg("start")
            .arg("--insecure")
            .arg("--listen-addr=localhost")
            .arg("--port=0")
            .arg("--http-port=0")
            .arg(format!("--listening-url-file={}", url_file.display()))
            .arg(format!("--store={}", tempdir.path().display()))
            .spawn()?;

        let mut url: String;
        loop {
            if url_file.exists() {
                match std::fs::read_to_string(url_file.clone()) {
                    Ok(s) => {
                        if s.contains("\n") {
                            url = s.trim().to_string();
                            break;
                        }
                    }
                    Err(_) => {}
                };
            }
            sleep(Duration::from_millis(10));
        }

        // Create test database and user.
        let conn = Connection::connect(url.clone(), TlsMode::None)?;
        conn.execute("CREATE DATABASE test", &[])?;

        Result::Ok(TempCockroach {
            tempdir: tempdir,
            process: process,
            url: url.replace("?sslmode=disable", "/test?sslmode=disable"),
        })
    }

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
