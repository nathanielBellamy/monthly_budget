use crate::biblio::money::tag::Tag;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Income {
    pub id: usize,
    pub active: bool,
    pub amount: usize,
    pub name: &'static str,
    pub recurrence_id: Option<usize>,
    pub tags: Option<Vec<Tag>>,
}
