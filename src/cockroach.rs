extern crate tempdir;

use std::io::Error;
use std::path::Path;
use std::process::Child;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use tempdir::TempDir;

pub struct TempCockroach {
    #[allow(dead_code)]
    tempdir: TempDir,

    process: Child,

    connection_string: String,
}

impl TempCockroach {
    pub fn new() -> Result<TempCockroach, Error> {
        let tempdir = TempDir::new("cockroach-data")?;

        let url_file = tempdir.path().join("url");
        let process = Command::new("cockroach")
            .arg("start")
            .arg("--insecure")
            .arg("--listen-addr=localhost")
            .arg("--port=0")
            .arg(format!("--listening-url-file={}", url_file.display()))
            .arg(format!("--store={}", tempdir.path().display()))
            .spawn()?;

        loop {
            if url_file.exists() {
                match std::fs::read_to_string(url_file.clone()) {
                    Ok(s) => {
                        if s.contains("\n") {
                            return Result::Ok(TempCockroach {
                                tempdir: tempdir,
                                process: process,
                                connection_string: s,
                            });
                        }
                    }
                    Err(_) => {}
                };
            }
            sleep(Duration::from_millis(10));
        }
    }

    pub fn connection_string(&self) -> &String {
        &self.connection_string
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
        let db = TempCockroach::new().expect("failed to start temporary CockroachDB");
        println!("{:?}", db.connection_string());
    }
}
