use rust_decimal::Decimal;
use crate::schema::expense::ExpenseStore;
use crate::schema::account::{Account, AccountStore};
use crate::schema::amount::{Amount, AmountStore};
use crate::store::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Payment {
    pub id: Option<usize>,
    pub completed_at: NaiveDateTime,
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

pub type PaymentStore = BTreeMap<usize, Payment>;

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

    pub fn expense_name(&self, store: &mut ExpenseStore) -> Option<String> {
      let mut name: Option<String> = None;
      for (id, exp) in store.iter() {
        if *id == self.expense_id {
          name = Some(exp.name.clone())
        }
      }
      name
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

    pub fn standard_amount(&self, store: &AmountStore) -> Option<Decimal> {
        match self.amount(store) {
            None => None,
            Some(amt) => Some(amt.standard),
        }
    }

    pub fn total(payment_store: PaymentStore, amount_store: &AmountStore) -> Decimal {
      let mut total = Decimal::new(00, 1);
      for (_id, payment) in payment_store.iter() {
        total += payment.standard_amount(amount_store).unwrap();
      }
      total
    }

    pub fn ids_by_account_id(account_id: usize, store: &mut PaymentStore) -> Vec<usize> {
      let mut payment_ids: Vec<usize> = vec![];
      for (id, payment) in store.iter() {
        if payment.account_id == account_id {
          payment_ids.push(*id);
        }
      }
      payment_ids
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
