use crate::schema::account::{Account, AccountStore};
use crate::schema::account_balance::AccountBalance;
use crate::schema::amount::{Amount, AmountStore};
use crate::store::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Payment {
    pub id: usize,
    pub completed_at: DateTime<Utc>,
    pub account_id: usize,
    pub amount_id: usize,
    pub expense_id: usize,
}

impl CsvRecord<Payment> for Payment {
    fn id(&self) -> usize {
        self.id
    }

    fn clone_record(&self) -> Payment {
        self.clone()
    }
}
impl CsvStore for Payment {}

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

    pub fn release_funds(&self, store: &mut Store) -> Result<(), Box<dyn Error>> {
        // create payment record
        store.payments.entry(self.id).or_insert(*self);

        // create account_balance record
        if let Some(acc) = self.from_account(&mut store.accounts) {
            let new_balance = AccountBalance {
                id: AccountBalance::new_id(&store.account_balances),
                account_id: self.account_id,
                reported_at: self.completed_at,
                amount: acc.current_balance(&store.account_balances).unwrap()
                    - self.standard_amount(&store.amounts).unwrap(),
            };
            store
                .account_balances
                .entry(new_balance.id)
                .or_insert(new_balance);
        }

        Ok(())
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
        assert_eq!(payment.account_id, from_acc.id);
    }

    #[test]
    #[allow(non_snake_case)]
    fn amount__returns_amount_by_id() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = Payment::by_id(1, &mut store.payments).unwrap();
        let amount = payment.amount(&store.amounts).unwrap();
        assert_eq!(payment.amount_id, amount.id)
    }

    #[test]
    #[allow(non_snake_case)]
    fn release_funds__creates_new_payment_record() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = Payment {
            id: Payment::new_id(&mut store.payments),
            completed_at: Utc::now(),
            account_id: 1,
            amount_id: 2,
            expense_id: 1,
        };
        let payment_count_curr = store.payments.len();
        payment.release_funds(&mut store).unwrap();
        assert_eq!(payment_count_curr + 1, store.payments.len());
    }

    #[test]
    #[allow(non_snake_case)]
    fn release_funds__creates_account_balance_record_with_correct_amount() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment = Payment::by_id(1, &mut store.payments).unwrap();
        let old_account_balance: f64 = payment
            .from_account(&store.accounts)
            .unwrap()
            .current_balance(&store.account_balances)
            .unwrap();
        let acc_bal_count_curr = store.account_balances.len();
        payment.release_funds(&mut store).unwrap();
        assert_eq!(acc_bal_count_curr + 1, store.account_balances.len());

        let new_account_balance =
            AccountBalance::by_id(acc_bal_count_curr + 1, &mut store.account_balances).unwrap();
        assert_eq!(
            new_account_balance.amount,
            old_account_balance - payment.standard_amount(&store.amounts).unwrap()
        );
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
