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
    pub name: String,
    pub base_url: String,
}

#[derive(Debug)]
pub struct Novel {
    pub name: String,
    pub author: String,
    // id of the novel
    pub id: u64,
    pub url: String,
    pub chapters: Vec<Chapter>

}

#[derive(Debug)]
pub struct Chapter {
    pub name: String,
    pub chapter_id: u64,    
}

