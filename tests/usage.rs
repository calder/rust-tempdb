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
