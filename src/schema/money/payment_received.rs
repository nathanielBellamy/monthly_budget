use crate::store::store::Store;
use crate::traits::csv_store::CsvStore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PaymentReceived {
    pub id: usize,
    pub completed_at: DateTime<Utc>,
    pub account_id: usize,
    pub income_id: usize,
    pub amount_id: usize,
}

impl PaymentReceived {
    pub fn _deposit_funds(&self, _store: &Store) -> Result<(), Box<dyn Error>> {
        // create payment_received record
        // create account_balance record
        Ok(())
    }
}

impl CsvStore for PaymentReceived {}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn release_funds_subtracts_ammount_from_acc() {}
}
