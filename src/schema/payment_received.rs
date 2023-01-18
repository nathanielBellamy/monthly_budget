use crate::schema::account::Account;
use crate::schema::account_balance::AccountBalance;
use crate::schema::amount::Amount;
use crate::store::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PaymentReceived {
    pub id: usize,
    pub completed_at: DateTime<Utc>,
    pub account_id: usize,
    pub income_id: usize,
    pub amount_id: usize,
}

impl CsvRecord<PaymentReceived> for PaymentReceived {
    fn id(&self) -> usize {
        self.id
    }

    fn clone_record(&self) -> PaymentReceived {
        self.clone()
    }
}
impl CsvStore for PaymentReceived {}

pub type PaymentReceivedStore = HashMap<usize, PaymentReceived>;

impl<'a, 'b: 'a> PaymentReceived {
    pub fn amount(&'a self, store: &'b Store) -> Option<&Amount> {
        let mut amount: Option<&Amount> = None;
        for (_id, amt) in store.amounts.iter() {
            if amt.id == self.amount_id {
                amount = Some(amt);
                break;
            }
        }
        amount
    }

    pub fn to_account(&'a self, store: &'b Store) -> Option<&Account> {
        let mut account: Option<&Account> = None;
        for (_id, acc) in store.accounts.iter() {
            if acc.id == self.account_id {
                account = Some(acc);
                break;
            }
        }
        account
    }

    pub fn deposit_funds(&self, store: &mut Store) -> Result<(), Box<dyn Error>> {
        store.payments_received.entry(self.id).or_insert(*self);

        // create account_balance record
        if let Some(acc) = self.to_account(store) {
            let new_balance = AccountBalance {
                id: AccountBalance::new_id(&store.account_balances),
                account_id: self.account_id,
                reported_at: self.completed_at,
                amount: acc.current_balance(store).unwrap() + self.standard_amount(store).unwrap(),
            };
            store
                .account_balances
                .entry(new_balance.id)
                .or_insert(new_balance);
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
mod payment_received_spec {
    use super::*;
    use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn to_account__returns_account_by_id() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment_rec = &store.payments_received[&0];
        let to_acc = payment_rec.to_account(&store).unwrap();
        assert_eq!(payment_rec.account_id, to_acc.id)
    }

    #[test]
    #[allow(non_snake_case)]
    fn amount__returns_amount_by_id() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment_rec = &store.payments_received[&0];
        let amount = payment_rec.amount(&store).unwrap();
        assert_eq!(payment_rec.amount_id, amount.id)
    }

    #[test]
    #[allow(non_snake_case)]
    fn deposit_funds__creates_new_payment_received_record() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment_rec = store.payments_received[&0].clone();
        let payment_rec_count_curr = store.payments_received.len();
        payment_rec.deposit_funds(&mut store).unwrap();
        assert_eq!(payment_rec_count_curr + 1, store.payments_received.len());
    }

    #[test]
    #[allow(non_snake_case)]
    fn release_funds__creates_account_balance_record_with_correct_amount() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment_rec = store.payments_received[&0].clone();
        let old_account_balance: f64 = payment_rec
            .to_account(&store)
            .unwrap()
            .current_balance(&store)
            .unwrap();
        let acc_bal_count_curr = store.account_balances.len();
        payment_rec.deposit_funds(&mut store).unwrap();
        assert_eq!(acc_bal_count_curr + 1, store.account_balances.len());

        let new_account_balance = &store.account_balances[&acc_bal_count_curr];
        assert_eq!(
            new_account_balance.amount,
            old_account_balance + payment_rec.standard_amount(&store).unwrap()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn standard_amount__returns_standard_field_of_associated_amount() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = &store.payments[&0];
        let amount = payment.amount(&store).unwrap();
        assert_eq!(payment.standard_amount(&store).unwrap(), amount.standard);
    }
}
