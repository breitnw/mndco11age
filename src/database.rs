use std::cmp::Ordering;
use crate::blog::Article;
use chrono::{DateTime, Utc};
use rusqlite::Connection;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Guest {
    pub name: String,
    pub timestamp: i64,
    pub date: String,
}

pub fn init() -> Result<(), rusqlite::Error> {
    // TODO: Log IPs so that each person can only post once
    let conn = Connection::open("data/db.sqlite")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS guests (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        timestamp INTEGER NOT NULL)",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS articles (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        tagline TEXT NOT NULL,
        timestamp INTEGER NOT NULL,
        location TEXT NOT NULL,
        preview TEXT NOT NULL,
        html TEXT NOT NULL)",
        (),
    )?;
    Ok(())
}

pub fn add_guest(name: &str) -> Result<(), rusqlite::Error> {
    let timestamp = Utc::now().timestamp();
    let conn = Connection::open("data/db.sqlite")?;
    conn.execute(
        "INSERT INTO guests (name, timestamp) VALUES (?1, ?2)",
        (name, timestamp),
    )?;
    Ok(())
}

pub fn get_guests() -> Result<Vec<Guest>, rusqlite::Error> {
    let conn = Connection::open("data/db.sqlite")?;
    let mut stmt = conn.prepare("SELECT name, timestamp FROM guests")?;
    let guest_iter = stmt.query_map([], |row| {
        Ok(Guest {
            name: row.get(0)?,
            timestamp: row.get(1)?,
            date: format_date(row.get(1)?),
        })
    })?;
    guest_iter.collect()
}

pub fn add_article(article: Article) -> Result<(), rusqlite::Error> {
    let conn = Connection::open("data/db.sqlite")?;
    conn.execute("INSERT INTO articles (title, tagline, timestamp, location, preview, html) VALUES \
    (?1, ?2, ?3, ?4, ?5, ?6)",
     (
         article.title,
         article.tagline,
         article.timestamp,
         article.location,
         article.preview,
         article.html)
    )?;
    Ok(())
}

pub fn get_articles() -> Result<Vec<Article>, rusqlite::Error> {
    let conn = Connection::open("data/db.sqlite")?;
    let mut stmt =
        conn.prepare("SELECT title, tagline, timestamp, location, preview, html FROM articles")?;
    let article_iter = stmt.query_map([], |row| {
        Ok(Article {
            title: row.get(0)?,
            tagline: row.get(1)?,
            timestamp: row.get(2)?,
            date: format_date(row.get(2)?),
            location: row.get(3)?,
            preview: row.get(4)?,
            html: row.get(5)?,
        })
    })?;

    // TODO: pretty nasty way to reverse the iterator, maybe find something better
    Ok(article_iter.collect::<Result<Vec<Article>, rusqlite::Error>>()?
        .into_iter()
        .rev()
        .collect())

}

pub fn get_article(location: &str) -> Result<Article, rusqlite::Error> {
    let conn = Connection::open("data/db.sqlite")?;
    let mut stmt =
        conn.prepare("SELECT title, tagline, timestamp, preview, html FROM articles WHERE location\
         = (?1)")?;
    let mut html_iter = stmt.query_map([location], |row| {
        Ok(Article {
            title: row.get(0)?,
            tagline: row.get(1)?,
            timestamp: row.get(2)?,
            date: format_date(row.get(2)?),
            location: location.to_string(),
            preview: row.get(3)?,
            html: row.get(4)?,
        })
    })?;
    html_iter
        .next()
        .unwrap_or(Err(rusqlite::Error::InvalidQuery))
}

pub(crate) fn format_date(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap()
        .format("%B %e, %Y")
        .to_string()
}
