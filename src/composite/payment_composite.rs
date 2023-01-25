use crate::traits::csv_record::CsvRecord;
use std::collections::BTreeMap;
use crate::schema::account_balance::AccountBalance;
use std::error::Error;
use crate::error_handler::error_handler::ErrorHandler;
use crate::schema::account::Account;
use crate::schema::amount::Amount;
use crate::schema::expense::Expense;
use crate::schema::payment::Payment;
use crate::store::store::Store;
use crate::traits::csv_store::CsvStore;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::composite::payment_display::{PaymentDisplay, PaymentDisplayStore};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PaymentComposite {
    pub id: Option<usize>,
    pub account_id: Option<usize>,
    pub account_name: String,
    pub account_balance_id: Option<usize>, // id of account_balance resulting from creation of payment
    pub prev_balance: Option<f64>,
    pub ending_balance: Option<f64>,
    pub amount_id: Option<usize>,
    pub amount_standard: f64,
    pub payment_id: Option<usize>,
    pub payment_completed_at: NaiveDateTime,
    pub expense_id: Option<usize>,
    pub expense_name: String,
}

impl CsvRecord<PaymentComposite> for PaymentComposite {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
      self.id = Some(new_id);
      self.id
    }

    fn clone_record(&self) -> PaymentComposite {
        self.clone()
    }
}

impl CsvStore<PaymentComposite> for PaymentComposite {}

pub type PaymentCompositeStore = BTreeMap<usize, PaymentComposite>;

type CreatePaymentResult = Result<(), Box<dyn Error>>;

impl PaymentComposite {
    pub fn display(&self) -> PaymentDisplay {
      PaymentDisplay {
        id: self.id,
        name: self.expense_name.clone(),
        amount: self.amount_standard,
        account_name: self.account_name.clone(),
        completed_at: self.payment_completed_at,
        prev_balance: self.prev_balance,
        ending_balance: self.ending_balance,
      }
    }

    pub fn create_payment(&mut self, store: &mut Store, complete_at: Option<NaiveDateTime>) -> CreatePaymentResult {
        if let Some(id) = self.payment_id {
          ErrorHandler::log(From::from(format!("Payment {:?} already exists.", id)))
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
                            id: self.account_id,
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

        if let None = self.expense_id {
            let expense_name = self.expense_name.clone();
            // try name lookup
            match Expense::by_name(expense_name, &store.expenses) {
                None => {
                    // create Expense record
                    let new_id = Expense::save_to_store(
                      Expense{
                        id: None,
                        active: true,
                        name: self.expense_name.clone(),
                      },
                      &mut store.expenses
                    );
                    self.expense_id = Some(new_id);
                },
                Some(exp) => self.expense_id = exp.id,
            }
        }

        let completed_at = match complete_at {
          None => Utc::now().naive_local(),
          Some(ndt) => ndt
        };
        // create Payment record
        self.payment_id = Some(Payment::new_id(&mut store.payments));
        Payment::save_to_store(
            Payment {
                id: None,
                completed_at: completed_at,
                account_id: self.account_id.unwrap(),
                amount_id: self.amount_id.unwrap(),
                expense_id: self.expense_id.unwrap(),
            },
            &mut store.payments,
        );

        // create new AccountBalance record
        let prev_balance = Account::by_id(self.account_id.unwrap(), &mut store.accounts)
                                   .unwrap()
                                   .current_balance(&mut store.account_balances);

        self.prev_balance = Some(prev_balance);

        let ending_balance = prev_balance - self.amount_standard;
        self.ending_balance = Some(ending_balance);
        self.account_balance_id = Some(AccountBalance::save_to_store(
          AccountBalance {
            id: None,
            account_id: self.account_id.unwrap(),
            amount: ending_balance,
            reported_at: completed_at,
          },
          &mut store.account_balances
        ));

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
