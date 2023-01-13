use crate::biblio::money::tag::Tag;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub id: usize,
    pub active: bool,
    pub name: String,
    pub recurrence_id: Option<usize>,
    pub tags: Option<Vec<Tag>>,
}

impl Expense {
    pub fn amount(&self) -> usize {
        //TODO: pass parameter, retrieve different data
        10
    }

    pub fn is_recurring(&self) -> bool {
        !self.recurrence_id.is_none()
    }

    pub fn read_csv_into_store(store: &mut Vec<Expense>) -> Result<&'static str, Box<dyn Error>> {
        // file handle and reader
        let path = std::env::current_dir()?;
        println!("{:?}", path.display());
        let file = File::open("data/expenses.csv")?;

        let mut reader = Reader::from_reader(file);

        // Check each result, return read errors
        for result in reader.deserialize() {
            match result {
                Err(err) => return Err(From::from(err)),
                Ok(record) => {
                    let expense: Expense = record;
                    println!("Expense: {:?}", expense);
                    store.push(expense);
                    println!("Expense Store: {:?}", store);
                }
            }
        }
        Ok("OK")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_recurring_returns_bool_based_on_recurrence_id() {
        let expenses = test_expenses();
        for expense in expenses.iter() {
            match expense.recurrence_id {
                Some(_id) => assert_eq!(expense.is_recurring(), true),
                None => assert_eq!(expense.is_recurring(), false),
            }
        }
    }

    fn test_expenses() -> [Expense; 2] {
        [
            Expense {
                id: 1,
                active: true,
                name: "Expense 1".to_string(),
                recurrence_id: Some(1),
                tags: None,
            },
            Expense {
                id: 2,
                active: true,
                name: "Expense 2".to_string(),
                recurrence_id: None,
                tags: None,
            },
        ]
    }
}
