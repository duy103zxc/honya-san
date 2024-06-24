use crate::model::{DataSource, Novel};

pub trait Source {
    fn metadata(&self) -> DataSource;
    // initial novel list
    fn fetching(&self, id: &str) -> Novel;
}
