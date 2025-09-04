use rusqlite::{Connection, params};
use chrono::Utc;
use crate::crypto::{KeyMaterial, encrypt};

pub struct HistoryDB { conn: Connection }

impl HistoryDB {
    pub fn open(path: &str) -> anyhow::Result<Self> { Ok(Self { conn: Connection::open(path)? }) }

    pub fn init(&self) -> anyhow::Result<()> {
        self.conn.execute("CREATE TABLE IF NOT EXISTS history(             id INTEGER PRIMARY KEY, ts INTEGER NOT NULL, url TEXT NOT NULL, title BLOB NOT NULL)", [])?;
        Ok(())
    }

    pub fn insert(&self, key: &KeyMaterial, url: &str, title: &str) -> anyhow::Result<()> {
        let enc_title = encrypt(key, title.as_bytes())?;
        self.conn.execute("INSERT INTO history(ts,url,title) VALUES(?,?,?)",
            params![Utc::now().timestamp(), url, enc_title])?;
        Ok(())
    }
}
