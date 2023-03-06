use crate::schema::account::{Account, AccountStore};
use crate::schema::account_balance::{AccountBalance, AccountBalanceStore};
use crate::schema::amount::{Amount, AmountStore};
use crate::schema::expense::{Expense, ExpenseStore};
use crate::schema::income::{Income, IncomeStore};
use crate::schema::payment::{Payment, PaymentStore};
use crate::schema::payment_received::{PaymentReceived, PaymentReceivedStore};
use crate::traits::csv_store::{CsvReadResult, CsvStore, CsvWriteResult};
use std::error::Error;

#[derive(Debug)]
pub struct Store {
    pub accounts: AccountStore,
    pub account_balances: AccountBalanceStore,
    pub amounts: AmountStore,
    pub expenses: ExpenseStore,
    pub incomes: IncomeStore,
    pub payments: PaymentStore,
    pub payments_received: PaymentReceivedStore,
}

pub type StoreInitResult<'a> = Result<&'a mut Store, Box<dyn Error>>;
pub type StoreWriteResult = Result<(), Box<dyn Error>>;

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

    pub fn init(&mut self, dir: Option<String>) -> StoreInitResult {
        let path: String = match dir {
            None => "data/".to_string(),
            Some(root) => root,
        };
        let import_res: [CsvReadResult; 7] = [
            Account::init_store(
                &mut self.accounts,
                format!("{path}{}", "accounts.csv").as_str(),
            ),
            AccountBalance::init_store(
                &mut self.account_balances,
                format!("{path}{}", "account_balances.csv").as_str(),
            ),
            Amount::init_store(
                &mut self.amounts,
                format!("{path}{}", "amounts.csv").as_str(),
            ),
            Expense::init_store(
                &mut self.expenses,
                format!("{path}{}", "expenses.csv").as_str(),
            ),
            Income::init_store(
                &mut self.incomes,
                format!("{path}{}", "incomes.csv").as_str(),
            ),
            Payment::init_store(
                &mut self.payments,
                format!("{path}{}", "payments.csv").as_str(),
            ),
            PaymentReceived::init_store(
                &mut self.payments_received,
                format!("{path}{}", "payments_received.csv").as_str(),
            ),
        ];

        for res in import_res.iter() {
            if let Err(err) = res {
                return Err(From::from(format!("Init Store Error: {err}")));
            }
        }
        Ok(self)
    }

    pub fn write_to_csv(&self, dir: Option<String>) -> StoreWriteResult {
        let path = dir.unwrap_or_else(|| "data/reports/".to_string());
        let write_res: [CsvWriteResult; 7] = [
            Account::write_to_csv(&self.accounts, format!("{path}{}", "accounts.csv").as_str()),
            AccountBalance::write_to_csv(
                &self.account_balances,
                format!("{path}{}", "account_balances.csv").as_str(),
            ),
            Amount::write_to_csv(&self.amounts, format!("{path}{}", "amounts.csv").as_str()),
            Expense::write_to_csv(&self.expenses, format!("{path}{}", "expenses.csv").as_str()),
            Income::write_to_csv(&self.incomes, format!("{path}{}", "incomes.csv").as_str()),
            Payment::write_to_csv(&self.payments, format!("{path}{}", "payments.csv").as_str()),
            PaymentReceived::write_to_csv(
                &self.payments_received,
                format!("{path}{}", "payments_received.csv").as_str(),
            ),
        ];

        for res in write_res.iter() {
            if let Err(err) = res {
                return Err(From::from(format!("Csv Store Write Error: {err}")));
            }
        }

        Ok(())
    }
}
