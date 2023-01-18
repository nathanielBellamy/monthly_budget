use crate::schema::account::Account;
use crate::schema::amount::Amount;
use crate::schema::expense::Expense;

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

impl PaymentComposite {
    pub fn create_payment(&self, store: &mut Store) -> () {
        let mut account: Option<&Account> = None;
        match account_id {
            None => {
                // try lookup by name
                match Account::by_name(self.account_name) {
                    None => {
                        // create Payment record
                        account = Account {
                            id: store.accounts.len(),
                            name: self.account_name,
                        };
                        Account::save_to_store(account);
                    }
                    Some(acc) => account = acc,
                }
            }
            Some(id) => account = Account::by_id(self.account_id), // failure here indicates id mismatch data
        }

        let mut amount: Option<&Amount> = None;
        match amount_id {
            None => {
                // create Amount record
                amount = Amount {
                    id: store.amounts.len(),
                    standard: amount_standard,
                };
                Amount::save_to_store(amount, store);
            }
            Some(id) => {
                // retrieve record
                amount = Amount::by_id(amount_id, store).unwrap(); // failure here indicates id mismatch data
            }
        }

        let mut expense: Option<&Expense> = None;
        match expense_id {
            None => {
                // try name lookup
                match Expense::by_name(expense_name.to_owned()){
                  None => { // create record
                    expense = Expense {
                    expense_id: store.expenses.len(),
                    active: true,
                    name: self.expense_name,
                  }
                  Expense::save_to_store(expense, store);
                  },
                  Some(exp) => expense = exp;
                }
            }
            Some(id) => {
              expense = Expense::by_id(id, store).unwrap(); // failure here indicates id mismatch data
            }
        }

        let mut payment: Option<&Payment> = None;
        match payment_id {
          None => { // create Payment record
            payment = Payment{
              id: store.payments.len(),
              completed_at: DateTime<Utc>,
              account_id: account.id,
              amount_id: amount.id,
              expense_id: expense.id,
            };
          },
          Some()
        }
    }
}

#[cfg(test)]
mod payment_composite_spec {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn create_payment__creates_account_if_none_exists() {
        let mut store = Store::new();
        Spec::init(&mut store);
    }
}
