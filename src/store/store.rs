use crate::biblio::money::account::Account;
use crate::biblio::money::account_balance::AccountBalance;
use crate::biblio::money::amount::Amount;
use crate::biblio::money::expense::Expense;
use crate::biblio::money::income::Income;
use crate::biblio::money::payment::Payment;
use crate::biblio::money::payment_received::PaymentReceived;
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

    pub fn init(&mut self) -> Result<&mut Store, Box<dyn Error>> {
        let import_res: [CsvReadResult; 7] = [
            Account::init_store_vec(&mut self.accounts, "data/accounts.csv"),
            AccountBalance::init_store_vec(&mut self.account_balances, "data/account_balances.csv"),
            Amount::init_store_vec(&mut self.amounts, "data/amounts.csv"),
            Expense::init_store_vec(&mut self.expenses, "data/expenses.csv"),
            Income::init_store_vec(&mut self.incomes, "data/incomes.csv"),
            Payment::init_store_vec(&mut self.payments, "data/payments.csv"),
            PaymentReceived::init_store_vec(
                &mut self.payments_received,
                "data/payments_received.csv",
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
