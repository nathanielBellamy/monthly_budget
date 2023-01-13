use crate::biblio::calendar;
use crate::biblio::money::expense::Expense;
use crate::biblio::money::income::Income;

mod biblio;

fn main() {
    let mut income_store: Vec<Income> = vec![];
    Income::read_csv_into_store(&mut income_store);

    let mut expense_store: Vec<Expense> = vec![];
    Expense::read_csv_into_store(&mut expense_store);
}
