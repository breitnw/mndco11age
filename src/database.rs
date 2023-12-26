use rusqlite::Connection;
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Guest {
    pub id: usize,
    pub name: String,
    pub timestamp: i64,
}

pub(crate) fn init() -> Result<(), rusqlite::Error> {
    // TODO: Log IPs so that each person can only post once
    let conn = Connection::open("data/db.sqlite")?;
    conn.execute("CREATE TABLE IF NOT EXISTS guests (id INTEGER PRIMARY KEY, name TEXT NOT NULL, timestamp INTEGER NOT NULL)",())?;
    Ok(())
}

pub(crate) fn add_guest(name: &str) -> Result<(), rusqlite::Error> {
    let timestamp = Utc::now().timestamp();
    let conn = Connection::open("data/db.sqlite")?;
    conn.execute("INSERT INTO guests (name, timestamp) VALUES (?1, ?2)", (name, timestamp))?;
    Ok(())
}

pub(crate) fn get_guests() -> Result<Vec<Guest>, rusqlite::Error> {
    let conn = Connection::open("data/db.sqlite")?;
    let mut stmt = conn.prepare("SELECT id, name, timestamp FROM guests")?;
    let guest_iter = stmt.query_map([], |row| {
        Ok(Guest {
            id: row.get(0)?,
            name: row.get(1)?,
            timestamp: row.get(2)?,
        })
    })?;
    guest_iter.collect()
}