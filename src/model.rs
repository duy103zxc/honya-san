pub struct DataSource {
    pub name: String,
    pub base_url: String,
}

#[derive(Debug)]
pub struct Novel {
    pub title: String,
    pub author: String,
    pub url: String,
    pub chapter_list: Vec<Chapter>,
}

#[derive(Debug)]
pub struct Chapter {
    pub chapter_id: String,
    pub name: String,
    pub content: Vec<String>,
}