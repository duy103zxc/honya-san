use crate::model::DataSource;

pub trait Source {
    fn metadata(&self) -> DataSource;
    fn fetching(&self, id: &str);
}
