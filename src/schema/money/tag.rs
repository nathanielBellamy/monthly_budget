use crate::traits::csv_store::CsvStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: usize,
    pub content: String,
}

impl CsvStore for Tag {}
