use crate::schema::account::{Account, AccountStore};
use crate::schema::amount::{Amount, AmountStore};
use crate::store::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Payment {
    pub id: Option<usize>,
    pub completed_at: DateTime<Utc>,
    pub account_id: usize,
    pub amount_id: usize,
    pub expense_id: usize,
}

impl CsvRecord<Payment> for Payment {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
      self.id = Some(new_id);
      self.id
    }

    fn clone_record(&self) -> Payment {
        self.clone()
    }
}
impl CsvStore<Payment> for Payment {}

pub type PaymentStore = HashMap<usize, Payment>;

impl<'a, 'b: 'a> Payment {
    pub fn amount(&'a self, store: &'b AmountStore) -> Option<Amount> {
        let mut amount: Option<Amount> = None;
        for (id, amt) in store.iter() {
            if *id == self.amount_id {
                amount = Some(*amt);
                break;
            }
        }
        amount
    }

    pub fn from_account(&'a self, store: &'b AccountStore) -> Option<Account> {
        let mut account: Option<Account> = None;
        for (id, acc) in store.iter() {
            if *id == self.account_id {
                account = Some(acc.clone_record());
                break;
            }
        }
        account
    }

    pub fn standard_amount(&self, store: &AmountStore) -> Option<f64> {
        match self.amount(store) {
            None => None,
            Some(amt) => Some(amt.standard),
        }
    }
}

#[cfg(test)]
mod payment_spec {
    use super::*;
    use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn from_account__returns_account_by_id() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = Payment::by_id(1, &mut store.payments).unwrap();
        let from_acc = payment.from_account(&store.accounts).unwrap();
        assert_eq!(payment.account_id, from_acc.id.unwrap());
    }

    #[test]
    #[allow(non_snake_case)]
    fn amount__returns_amount_by_id() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = Payment::by_id(1, &mut store.payments).unwrap();
        let amount = payment.amount(&store.amounts).unwrap();
        assert_eq!(payment.amount_id, amount.id.unwrap())
    }

    #[test]
    #[allow(non_snake_case)]
    fn standard_amount__returns_standard_field_of_associated_amount() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = Payment::by_id(1, &mut store.payments).unwrap();
        let amount = payment.amount(&store.amounts).unwrap();
        assert_eq!(
            payment.standard_amount(&store.amounts).unwrap(),
            amount.standard
        );
    }
}
