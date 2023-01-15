use crate::traits::csv_store::CsvStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: usize,
    pub name: String,
}

impl Account {
    pub fn release_funds(&self, _from_acc: &Account) -> usize {
        //
        0
    }
}

impl CsvStore for Account {}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn release_funds_subtracts_ammount_from_acc() {}
}
