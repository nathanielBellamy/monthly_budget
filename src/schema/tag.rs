use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: Option<usize>,
    pub content: String,
}

#[allow(unused)]
pub type TagStore = BTreeMap<usize, Tag>;

impl CsvRecord<Tag> for Tag {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
      self.id = Some(new_id);
      self.id
    }

    fn clone_record(&self) -> Tag {
        Tag {
            id: self.id,
            content: self.content.clone(),
        }
    }
}
impl CsvStore<Tag> for Tag {}
