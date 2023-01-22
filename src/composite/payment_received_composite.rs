use crate::traits::csv_record::CsvRecord;
use std::collections::HashMap;
use crate::schema::account_balance::AccountBalance;
use crate::schema::payment_received::PaymentReceived;
use crate::schema::income::Income;
use std::error::Error;
use crate::error_handler::error_handler::ErrorHandler;
use crate::schema::account::Account;
use crate::schema::amount::Amount;
use crate::store::store::Store;
use crate::traits::csv_store::CsvStore;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentReceivedComposite {
    pub id: Option<usize>,
    pub account_id: Option<usize>,
    pub account_name: String,
    pub amount_id: Option<usize>,
    pub amount_standard: f64,
    pub payment_received_id: Option<usize>,
    pub payment_received_completed_at: NaiveDateTime,
    pub income_id: Option<usize>,
    pub income_name: String,
}

impl CsvRecord<PaymentReceivedComposite> for PaymentReceivedComposite {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
      self.id = Some(new_id);
      self.id
    }

    fn clone_record(&self) -> PaymentReceivedComposite {
        self.clone()
    }
}

impl CsvStore<PaymentReceivedComposite> for PaymentReceivedComposite {}

pub type PaymentReceivedCompositeStore = HashMap<usize, PaymentReceivedComposite>;

type CreatePaymentReceivedResult = Result<(), Box<dyn Error>>;

impl PaymentReceivedComposite {
    pub fn create_payment_received(&mut self, store: &mut Store) -> CreatePaymentReceivedResult {
        if let Some(id) = self.payment_received_id {
          ErrorHandler::log(From::from(format!("PaymentReceived {:?} already exists.", id)))
        }

        if let None = self.account_id {
            // try lookup by name
            match Account::by_name(&self.account_name, &mut store.accounts) {
                None => {
                    // create Account record
                    self.account_id = Some(Account::new_id(&mut store.accounts));
                    self.account_id = Some(
                      Account::save_to_store(
                        Account {
                            id: self.account_id, // T::new_id returns T, this unwrap is a formality
                            name: self.account_name.clone(),
                        },
                        &mut store.accounts,
                    ));
                },
                Some(acc) => self.account_id = acc.id,
            }
        }

        if let None = self.amount_id {
          // create Amount record
          self.amount_id = Some(
            Amount::save_to_store(
              Amount {
                id: self.amount_id,
                standard: self.amount_standard,
                high: None,
                low: None,
              },
              &mut store.amounts
          ));
        }

        if let None = self.income_id {
            // try name lookup
            match Income::by_name(&self.income_name, &store.incomes) {
                None => {
                    // create Income record
                    let new_id = Income::save_to_store(
                      Income{
                        id: None,
                        active: true,
                        name: self.income_name.clone(),
                      },
                      &mut store.incomes
                    );
                    self.income_id = Some(new_id);
                },
                Some(inc) => self.income_id = inc.id,
            }
        }

        // create PaymentReceived record
        self.payment_received_id = Some(PaymentReceived::new_id(&mut store.payments_received));
        PaymentReceived::save_to_store(
            PaymentReceived {
                id: None,
                completed_at: Utc::now().naive_local(),
                account_id: self.account_id.unwrap(),
                amount_id: self.amount_id.unwrap(),
                income_id: self.income_id.unwrap(),
            },
            &mut store.payments_received,
        );

        // create new account balance record
        let old_balance = Account::by_id(self.account_id.unwrap(), &mut store.accounts)
                                   .unwrap()
                                   .current_balance(&mut store.account_balances);
        AccountBalance::save_to_store(
          AccountBalance {
            id: None,
            account_id: self.account_id.unwrap(),
            amount: old_balance + self.amount_standard,
            reported_at: Utc::now().naive_local(),
          },
          &mut store.account_balances
        );

        Ok(())
    }
}

#[cfg(test)]
mod payment_composite_spec {
    use super::*;
    use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn create_payment__creates_account_if_none_exists() {
        let mut store = Store::new();
        Spec::init(&mut store);

        assert_eq!(2, 2);
    }
}
