use crate::schema::money::account::Account;
use crate::schema::money::account_balance::AccountBalance;
use crate::schema::money::amount::Amount;
use crate::schema::money::expense::Expense;
use crate::schema::money::income::Income;
use crate::schema::money::payment::Payment;
use crate::schema::money::payment_received::PaymentReceived;
use crate::traits::csv_store::{CsvReadResult, CsvStore};
use std::error::Error;

#[derive(Debug)]
pub struct Store {
    pub accounts: Vec<Account>,
    pub account_balances: Vec<AccountBalance>,
    pub amounts: Vec<Amount>,
    pub expenses: Vec<Expense>,
    pub incomes: Vec<Income>,
    pub payments: Vec<Payment>,
    pub payments_received: Vec<PaymentReceived>,
}

pub type StoreInitResult<'a> = Result<&'a mut Store, Box<dyn Error>>;

impl Store {
    pub fn new() -> Store {
        Store {
            accounts: vec![],
            account_balances: vec![],
            amounts: vec![],
            expenses: vec![],
            incomes: vec![],
            payments: vec![],
            payments_received: vec![],
        }
    }

    pub fn init(&mut self, data_root: Option<&'static str>) -> StoreInitResult {
        let path: &str;
        match data_root {
            None => path = "data/",
            Some(root) => path = root,
        }
        let import_res: [CsvReadResult; 7] = [
            Account::init_store_vec(
                &mut self.accounts,
                format!("{}{}", path, "accounts.csv").as_str(),
            ),
            AccountBalance::init_store_vec(
                &mut self.account_balances,
                format!("{}{}", path, "account_balances.csv").as_str(),
            ),
            Amount::init_store_vec(
                &mut self.amounts,
                format!("{}{}", path, "amounts.csv").as_str(),
            ),
            Expense::init_store_vec(
                &mut self.expenses,
                format!("{}{}", path, "expenses.csv").as_str(),
            ),
            Income::init_store_vec(
                &mut self.incomes,
                format!("{}{}", path, "incomes.csv").as_str(),
            ),
            Payment::init_store_vec(
                &mut self.payments,
                format!("{}{}", path, "payments.csv").as_str(),
            ),
            PaymentReceived::init_store_vec(
                &mut self.payments_received,
                format!("{}{}", path, "payments_received.csv").as_str(),
            ),
        ];

        for res in import_res.iter() {
            if let Err(err) = res {
                return Err(From::from(format!("Init Store Error: {:?}", err)));
            }
        }
        Ok(self)
    }
}
