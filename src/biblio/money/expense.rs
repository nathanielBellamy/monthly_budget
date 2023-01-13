use crate::traits::csv_store::CsvStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub id: usize,
    pub active: bool,
    pub name: String,
    pub recurrence_id: Option<usize>,
}

impl Expense {
    pub fn amount(&self) -> usize {
        //TODO: pass parameter, retrieve different data
        10
    }

    pub fn is_recurring(&self) -> bool {
        !self.recurrence_id.is_none()
    }
}

impl CsvStore for Expense {}

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
            },
            Expense {
                id: 2,
                active: true,
                name: "Expense 2".to_string(),
                recurrence_id: None,
            },
        ]
    }
}
