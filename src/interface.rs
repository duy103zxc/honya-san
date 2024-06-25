use crate::model::{DataSource, Novel};

pub trait Source {
    fn metadata(&self) -> DataSource;
    fn fetching(&self, id: &str) -> Novel;
}
