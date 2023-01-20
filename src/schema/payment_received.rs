use crate::schema::account::Account;
use crate::schema::amount::{Amount, AmountStore};
use crate::store::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct PaymentReceived {
    pub id: Option<usize>,
    pub completed_at: DateTime<Utc>,
    pub account_id: usize,
    pub income_id: usize,
    pub amount_id: usize,
}

impl CsvRecord<PaymentReceived> for PaymentReceived {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
      self.id = Some(new_id);
      self.id
    }

    fn clone_record(&self) -> PaymentReceived {
        self.clone()
    }
}
impl CsvStore<PaymentReceived> for PaymentReceived {}

pub type PaymentReceivedStore = HashMap<usize, PaymentReceived>;

impl<'a, 'b: 'a> PaymentReceived {
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

    pub fn to_account(&'a self, store: &'b Store) -> Option<&Account> {
        let mut account: Option<&Account> = None;
        for (id, acc) in store.accounts.iter() {
            if *id == self.account_id {
                account = Some(acc);
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
mod payment_received_spec {
    use super::*;
    use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn to_account__returns_account_by_id() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment_rec = PaymentReceived::by_id(1, &mut store.payments_received).unwrap();
        let to_acc = payment_rec.to_account(&store).unwrap();
        assert_eq!(payment_rec.account_id, to_acc.id.unwrap())
    }

    #[test]
    #[allow(non_snake_case)]
    fn amount__returns_amount_by_id() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment_rec = PaymentReceived::by_id(1, &mut store.payments_received).unwrap();
        let amount = payment_rec.amount(&store.amounts).unwrap();
        assert_eq!(payment_rec.amount_id, amount.id.unwrap())
    }

    #[test]
    #[allow(non_snake_case)]
    fn standard_amount__returns_standard_field_of_associated_amount() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment_received = PaymentReceived::by_id(1, &mut store.payments_received).unwrap();
        let amount = payment_received.amount(&store.amounts).unwrap();
        assert_eq!(
            payment_received.standard_amount(&store.amounts).unwrap(),
            amount.standard
        );
    }
}
