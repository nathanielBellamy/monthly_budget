use crate::traits::csv_store::CsvStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Income {
    pub id: usize,
    pub active: bool,
    pub name: String,
}

impl Income {
    pub fn amount(&self) -> usize {
        //TODO: accept different parameters to make different computations
        10
    }
}

impl CsvStore for Income {}
