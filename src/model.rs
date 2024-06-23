use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub enum Status {
    Unknown,
    Ongoing,
    Complete,
    Discontinued,
}

pub struct DataSource {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub version: u64,
    pub base_url: String,
}

#[derive(Debug)]
pub struct Novel {
    // id of the source
    pub source_id: u64,
    // id of the novel
    pub id: u64,
    pub name: String,
    pub url: String,
    pub chapter_count: u64,
}

pub struct Chapter {
    pub id: u64,
    pub novel_id: u64,
    pub url: String,
    pub name: String,
    pub content: String,
}

