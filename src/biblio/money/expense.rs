use crate::biblio::money::tag::Tag;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub id: usize,
    pub active: bool,
    pub amount: usize,
    pub name: &'static str,
    pub recurrence_id: Option<usize>,
    pub tags: Option<Vec<Tag>>,
}

impl<'a> Expense {
    pub fn is_recurring(&self) -> bool {
        !self.recurrence_id.is_none()
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
                amount: 1_000,
                name: "Expense 1",
                recurrence_id: Some(1),
                tags: None,
            },
            Expense {
                id: 2,
                active: true,
                amount: 2_000,
                name: "Expense 2",
                recurrence_id: None,
                tags: None,
            },
        ]
    }
}
