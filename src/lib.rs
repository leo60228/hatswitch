use chrono::{DateTime, Utc};

pub mod parser;
pub use parser::gamestate as parse;

pub mod writer;
pub use writer::gamestate as write;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState<'a> {
    pub entries: Vec<Entry<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry<'a> {
    pub typ: EntryType,
    pub create: DateTime<Utc>,
    pub access: DateTime<Utc>,
    pub modify: DateTime<Utc>,
    pub name: String,
    pub data: &'a [u8],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EntryType {
    File,
    Directory,
}
