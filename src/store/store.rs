use crate::biblio::money::account::Account;
use crate::biblio::money::account_balance::AccountBalance;
use crate::biblio::money::amount::Amount;
use crate::biblio::money::expense::Expense;
use crate::biblio::money::income::Income;
use crate::biblio::money::payment::Payment;
use crate::biblio::money::payment_received::PaymentReceived;
use crate::traits::csv_store::CsvStore;
use std::error::Error;

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

    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let account_res = Account::init_store_vec(&mut self.accounts, "data/accounts.csv");
        let account_balance_res =
            AccountBalance::init_store_vec(&mut self.account_balances, "data/account_balances.csv");
        let amount_res = Amount::init_store_vec(&mut self.amounts, "data/amounts.csv");
        let expense_res = Expense::init_store_vec(&mut self.expenses, "data/expenses.csv");
        let income_res = Income::init_store_vec(&mut self.incomes, "data/incomes.csv");
        let payment_res = Payment::init_store_vec(&mut self.payments, "data/payments.csv");
        let payments_received_res = PaymentReceived::init_store_vec(
            &mut self.payments_received,
            "data/payments_received.csv",
        );

        let import_res = vec![
            account_res,
            account_balance_res,
            amount_res,
            expense_res,
            income_res,
            payment_res,
            payments_received_res,
        ];

        for res in import_res.iter() {
            if let Err(err) = res {
                println!("{:?}", err);
            }
        }
        Ok(())
    }
}
