use crate::biblio::calendar;
use crate::biblio::money::expense::Expense;
use crate::biblio::money::income::Income;
use crate::traits::csv_store::CsvStore; // init_csv_store

mod biblio;
mod traits;

fn main() {
    let mut income_store: Vec<Income> = vec![];
    Income::init_store_vec(&mut income_store, "data/incomes.csv");

    let mut expense_store: Vec<Expense> = vec![];
    Expense::init_store_vec(&mut expense_store, "data/expenses.csv");
}
