extern crate tempdb;

use tempdb::cockroach::TempCockroach;

#[test]
fn test() {
    let db = TempCockroach::new().expect("failed to create DB");
    println!("{:?}", db.connection_string());
}
