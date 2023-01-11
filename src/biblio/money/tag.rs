use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    id: u64,

    #[serde(borrow)]
    pub content: &'static str,
}
