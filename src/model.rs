#[derive(Debug)]
pub struct DataSource {
    pub name: String,
    pub base_url: String,
}

#[derive(Debug)]
pub struct Novel {
    pub name: String,
    pub author: String,
    pub id: u64,
    pub url: String,
    pub chapters: Vec<String>

}

#[derive(Debug)]
pub struct Chapter {
    pub name: String,
    pub chapter_id: String,    
}

