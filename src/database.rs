use rusqlite::Connection;
use chrono::Utc;
use serde::Serialize;
use crate::blog::Article;

#[derive(Serialize, Debug)]
pub struct Guest {
    pub name: String,
    pub timestamp: i64,
}

pub fn init() -> Result<(), rusqlite::Error> {
    // TODO: Log IPs so that each person can only post once
    let conn = Connection::open("data/db.sqlite")?;
    conn.execute("CREATE TABLE IF NOT EXISTS guests (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        timestamp INTEGER NOT NULL)",())?;
    conn.execute("CREATE TABLE IF NOT EXISTS articles (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        timestamp INTEGER NOT NULL,
        location TEXT NOT NULL,
        preview TEXT NOT NULL,
        html TEXT NOT NULL)",())?;
    Ok(())
}

pub fn add_guest(name: &str) -> Result<(), rusqlite::Error> {
    let timestamp = Utc::now().timestamp();
    let conn = Connection::open("data/db.sqlite")?;
    conn.execute("INSERT INTO guests (name, timestamp) VALUES (?1, ?2)", (name, timestamp))?;
    Ok(())
}

pub fn get_guests() -> Result<Vec<Guest>, rusqlite::Error> {
    let conn = Connection::open("data/db.sqlite")?;
    let mut stmt = conn.prepare("SELECT name, timestamp FROM guests")?;
    let guest_iter = stmt.query_map([], |row| {
        Ok(Guest {
            name: row.get(0)?,
            timestamp: row.get(1)?,
        })
    })?;
    guest_iter.collect()
}

pub fn add_article(article: Article) -> Result<(), rusqlite::Error> {
    let timestamp = Utc::now().timestamp();
    let conn = Connection::open("data/db.sqlite")?;

    conn.execute("INSERT INTO articles (title, timestamp, location, preview, html) VALUES (?1, ?2, ?3, ?4, ?5)",
                 (article.title, article.timestamp, article.location, article.preview, article.html))?;
    Ok(())
}

pub fn get_articles() -> Result<Vec<Article>, rusqlite::Error> {
    let conn = Connection::open("data/db.sqlite")?;
    let mut stmt = conn.prepare("SELECT title, timestamp, location, preview, html FROM articles")?;
    let article_iter = stmt.query_map([], |row| {
        Ok(Article {
            title: row.get(0)?,
            timestamp: row.get(1)?,
            location: row.get(2)?,
            preview: row.get(3)?,
            html: row.get(4)?,
        })
    })?;
    article_iter.collect()
}

pub fn get_article(location: &str) -> Result<Article, rusqlite::Error> {
    let conn = Connection::open("data/db.sqlite")?;
    let mut stmt = conn.prepare("SELECT title, timestamp, preview, html FROM articles WHERE location = (?1)")?;
    let mut html_iter = stmt.query_map(
        [location], |row| Ok(Article {
            title: row.get(0)?,
            timestamp: row.get(1)?,
            location: location.to_string(),
            preview: row.get(2)?,
            html: row.get(3)?,
        })
    )?;
    html_iter
        .next()
        .unwrap_or(Err(rusqlite::Error::InvalidQuery))
}
