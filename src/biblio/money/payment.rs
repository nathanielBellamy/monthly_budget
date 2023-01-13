use crate::traits::csv_store::CsvStore;
use crate::biblio::money::account::Account;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    pub id: usize,
    pub date: DateTime<Utc>,
    pub amount_id: usize,
    pub expense_id: usize,
}

impl Payment {
    pub fn release_funds(&self, _from_account: &Account) -> usize {
        //
        0
    }
}

impl CsvStore for Payment {}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn release_funds_subtracts_ammount_from_acc() {}
}
