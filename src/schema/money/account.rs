use crate::schema::money::account_balance::AccountBalance;
use crate::store::store::Store;
use crate::traits::csv_store::CsvStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: usize,
    pub name: String,
}

impl CsvStore for Account {}

impl Account {
    pub fn current_balance(&self, store: &Store) -> Option<f64> {
        let mut balance: Option<&AccountBalance> = None;
        for bal in store.account_balances.iter().rev() {
            // most recently pushed balance
            if bal.account_id == self.id {
                balance = Some(bal);
                break;
            }
        }
        match balance {
            None => None,
            Some(bal) => Some(bal.amount),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn release_funds_subtracts_ammount_from_acc() {}
}
