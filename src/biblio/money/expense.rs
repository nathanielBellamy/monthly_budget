use crate::traits::csv_store::CsvStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub id: usize,
    pub active: bool,
    pub name: String,
}

impl Expense {
    pub fn amount(&self) -> usize {
        //TODO: pass parameter, retrieve different data
        10
    }
}

impl CsvStore for Expense {}

#[cfg(test)]
mod tests {
    // use super::*;
}
