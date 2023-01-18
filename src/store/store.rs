use crate::schema::account::{Account, AccountStore};
use crate::schema::account_balance::{AccountBalance, AccountBalanceStore};
use crate::schema::amount::{Amount, AmountStore};
use crate::schema::expense::{Expense, ExpenseStore};
use crate::schema::income::{Income, IncomeStore};
use crate::schema::payment::{Payment, PaymentStore};
use crate::schema::payment_received::{PaymentReceived, PaymentReceivedStore};
use crate::traits::csv_store::{CsvReadResult, CsvStore};
use std::collections::HashMap;
use std::error::Error;

//TODO: HashMap's instead of Vec
#[derive(Debug)]
pub struct Store {
    pub accounts: HashMap<usize, Account>,
    pub account_balances: HashMap<usize, AccountBalance>,
    pub amounts: HashMap<usize, Amount>,
    pub expenses: HashMap<usize, Expense>,
    pub incomes: HashMap<usize, Income>,
    pub payments: HashMap<usize, Payment>,
    pub payments_received: HashMap<usize, PaymentReceived>,
}

pub type StoreInitResult<'a> = Result<&'a mut Store, Box<dyn Error>>;

// TODO: wrap store in RefCell
impl Store {
    pub fn new() -> Store {
        Store {
            accounts: AccountStore::new(),
            account_balances: AccountBalanceStore::new(),
            amounts: AmountStore::new(),
            expenses: ExpenseStore::new(),
            incomes: IncomeStore::new(),
            payments: PaymentStore::new(),
            payments_received: PaymentReceivedStore::new(),
        }
    }

    pub fn init(&mut self, data_root: Option<&'static str>) -> StoreInitResult {
        let path: &str;
        match data_root {
            None => path = "data/",
            Some(root) => path = root,
        }
        let import_res: [CsvReadResult; 7] = [
            Account::init_store(
                &mut self.accounts,
                format!("{}{}", path, "accounts.csv").as_str(),
            ),
            AccountBalance::init_store(
                &mut self.account_balances,
                format!("{}{}", path, "account_balances.csv").as_str(),
            ),
            Amount::init_store(
                &mut self.amounts,
                format!("{}{}", path, "amounts.csv").as_str(),
            ),
            Expense::init_store(
                &mut self.expenses,
                format!("{}{}", path, "expenses.csv").as_str(),
            ),
            Income::init_store(
                &mut self.incomes,
                format!("{}{}", path, "incomes.csv").as_str(),
            ),
            Payment::init_store(
                &mut self.payments,
                format!("{}{}", path, "payments.csv").as_str(),
            ),
            PaymentReceived::init_store(
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
