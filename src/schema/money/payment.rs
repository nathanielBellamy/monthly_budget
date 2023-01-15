use crate::schema::money::account::Account;
use crate::schema::money::account_balance::AccountBalance;
use crate::schema::money::amount::Amount;
use crate::store::store::Store;
use crate::traits::csv_store::CsvStore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Payment {
    pub id: usize,
    pub completed_at: DateTime<Utc>,
    pub account_id: usize,
    pub amount_id: usize,
    pub expense_id: usize,
}

impl CsvStore for Payment {}

impl<'a, 'b: 'a> Payment {
    pub fn from_account(&'a self, store: &'b Store) -> Option<&Account> {
        let mut account: Option<&Account> = None;
        for acc in store.accounts.iter() {
            if acc.id == self.account_id {
                account = Some(acc);
                break;
            }
        }
        account
    }

    pub fn amount(&'a self, store: &'b Store) -> Option<&Amount> {
        let mut amount: Option<&Amount> = None;
        for amt in store.amounts.iter() {
            if amt.id == self.amount_id {
                amount = Some(amt);
                break;
            }
        }
        amount
    }

    pub fn release_funds(&self, store: &mut Store) -> Result<(), Box<dyn Error>> {
        // create payment record
        store.payments.push(*self);

        // create account_balance record
        if let Some(acc) = self.from_account(store) {
            let new_balance = AccountBalance {
                id: store.accounts.len(),
                account_id: self.account_id,
                reported_at: self.completed_at,
                amount: acc.current_balance(store).unwrap() - self.standard_amount(store).unwrap(),
            };
            store.account_balances.push(new_balance)
        }

        Ok(())
    }

    pub fn standard_amount(&self, store: &Store) -> Option<f64> {
        match self.amount(store) {
            None => None,
            Some(amt) => Some(amt.standard),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn from_account__returns_account_by_id() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = &store.payments[0];
        let from_acc = payment.from_account(&store).unwrap();
        assert_eq!(payment.account_id, from_acc.id)
    }

    #[test]
    #[allow(non_snake_case)]
    fn amount__returns_amount_by_id() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = &store.payments[0];
        let amount = payment.amount(&store).unwrap();
        assert_eq!(payment.amount_id, amount.id)
    }

    #[test]
    #[allow(non_snake_case)]
    fn release_funds__creates_new_payment_record() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = store.payments[0].clone();
        let payment_count_curr = store.payments.len();
        payment.release_funds(&mut store).unwrap();
        assert_eq!(payment_count_curr + 1, store.payments.len());
    }

    #[test]
    #[allow(non_snake_case)]
    fn release_funds__creates_account_balance_record() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = store.payments[0].clone();
        let acc_bal_count_curr = store.account_balances.len();
        payment.release_funds(&mut store).unwrap();
        assert_eq!(acc_bal_count_curr + 1, store.account_balances.len());
    }

    #[test]
    #[allow(non_snake_case)]
    fn standard_amount__returns_standard_field_of_associated_amount() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = &store.payments[0];
        let amount = payment.amount(&store).unwrap();
        assert_eq!(payment.standard_amount(&store).unwrap(), amount.standard);
    }
}
