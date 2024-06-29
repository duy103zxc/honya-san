use crate::model::{Chapter, DataSource, Novel};

pub trait Source {
    fn metadata(&self) -> DataSource;
    fn fetch_novel(&self, novel_id: &str) -> Novel;
    fn fetch_chapter(&self, chapter_id: &str) -> Chapter;
}