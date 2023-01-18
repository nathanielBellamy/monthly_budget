use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: usize,
    pub content: String,
}

pub type TagStore = HashMap<usize, Tag>;

impl CsvRecord<Tag> for Tag {
    fn id(&self) -> usize {
        self.id
    }

    fn clone_record(&self) -> Tag {
        Tag {
            id: self.id,
            content: self.content.clone(),
        }
    }
}
impl CsvStore for Tag {}
