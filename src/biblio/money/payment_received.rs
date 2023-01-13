use crate::traits::csv_store::CsvStore;
use crate::biblio::money::account::Account;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentReceived {
    pub id: usize,
    pub date: DateTime<Utc>,
    pub income_id: usize,
    pub amount_id: usize,
}

impl PaymentReceived {
    pub fn deposit_funds(&self, _to_account: &Account) -> usize {
        //
        0
    }
}

impl CsvStore for PaymentReceived {}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn release_funds_subtracts_ammount_from_acc() {}
}
