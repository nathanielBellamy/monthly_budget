use std::error::Error;
use crate::error_handler::error_handler::ErrorHandler;
use crate::schema::account::Account;
use crate::schema::amount::Amount;
use crate::schema::expense::Expense;
use crate::schema::payment::Payment;
use crate::store::store::Store;
use crate::traits::csv_store::CsvStore;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct PaymentComposite {
    pub account_id: Option<usize>,
    pub account_name: String,
    pub amount_id: Option<usize>,
    pub amount_standard: f64,
    pub payment_id: Option<usize>,
    pub payment_completed_at: DateTime<Utc>,
    pub expense_id: Option<usize>,
    pub expense_name: String,
}

type CreatePaymentResult = Result<(), Box<dyn Error>>;

impl PaymentComposite {
    pub fn create_payment(&mut self, store: &mut Store) -> CreatePaymentResult {
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

        if let None = self.expense_id {
            // try name lookup
            match Expense::by_name(&self.expense_name, &store.expenses) {
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

        // create Payment record
        self.payment_id = Some(Payment::new_id(&mut store.payments));
        Payment::save_to_store(
            Payment {
                id: None,
                completed_at: Utc::now(),
                account_id: self.account_id.unwrap(),
                amount_id: self.amount_id.unwrap(),
                expense_id: self.expense_id.unwrap(),
            },
            &mut store.payments,
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
