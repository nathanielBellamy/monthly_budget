use crate::biblio::money::expense::Expense;
use crate::biblio::money::income::Income;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Month {
    pub id: u16,
    pub key: MonthKey,
    pub budget: usize,

    #[serde(bound(deserialize = "Income: Deserialize<'de>"))]
    pub incomes: Vec<Income>,
    #[serde(bound(deserialize = "Expense: Deserialize<'de>"))]
    pub expenses: Vec<Expense>,
    pub savings_at_start: usize,
}

impl Month {
    pub fn gross_income(&self) -> usize {
        let mut res: usize = 0;
        for income in self.incomes.iter() {
            if income.active {
                res += income.amount();
            }
        }
        res
    }

    pub fn gross_expense(&self) -> usize {
        let mut res: usize = 0;
        for expense in self.expenses.iter() {
            if expense.active {
                res += expense.amount();
            }
        }
        res
    }

    pub fn net_income(&self) -> usize {
        self.gross_income() - self.gross_expense()
    }

    pub fn display_name(&self) -> &str {
        match self.key {
            MonthKey::Jan => "January",
            MonthKey::Feb => "February",
            MonthKey::Mar => "March",
            MonthKey::Apr => "April",
            MonthKey::May => "May",
            MonthKey::Jun => "June",
            MonthKey::Jul => "July",
            MonthKey::Aug => "August",
            MonthKey::Sep => "September",
            MonthKey::Oct => "October",
            MonthKey::Nov => "November",
            MonthKey::Dec => "December",
        }
    }

    pub fn display_number(&self) -> &str {
        match self.key {
            MonthKey::Jan => "01",
            MonthKey::Feb => "02",
            MonthKey::Mar => "03",
            MonthKey::Apr => "04",
            MonthKey::May => "05",
            MonthKey::Jun => "06",
            MonthKey::Jul => "07",
            MonthKey::Aug => "08",
            MonthKey::Sep => "09",
            MonthKey::Oct => "10",
            MonthKey::Nov => "11",
            MonthKey::Dec => "12",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MonthKey {
    Jan,
    Feb,
    Mar,
    Apr,
    May,
    Jun,
    Jul,
    Aug,
    Sep,
    Oct,
    Nov,
    Dec,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gross_income_sums_active_income_amounts() {
        let month = month();

        assert_eq!(month.gross_income(), 4_000)
    }

    #[test]
    fn gross_expense_sums_active_expense_amounts() {
        let month = month();

        assert_eq!(month.gross_income(), 4_000)
    }

    #[test]
    fn net_income_diffs_gross_income_and_gross_expense() {
        let month = month();

        assert_eq!(month.net_income(), 0)
    }

    fn month() -> Month {
        let data: SeedData = seed_data();

        Month {
            incomes: data.incomes,
            expenses: data.expenses,
            id: 1,
            key: MonthKey::Jan,
            budget: 5_000,
            savings_at_start: 10_000,
        }
    }

    struct SeedData {
        incomes: Vec<Income>,
        expenses: Vec<Expense>,
    }

    fn seed_data() -> SeedData {
        let mut incomes: Vec<Income> = Vec::new();
        incomes.push(Income {
            id: 1,
            active: true,
            name: "Income 1".to_string(),
            recurrence_id: None,
            tags: None,
        });
        incomes.push(Income {
            id: 2,
            active: false,
            name: "Income 2".to_string(),
            recurrence_id: None,
            tags: None,
        });
        incomes.push(Income {
            id: 3,
            active: true,
            name: "Income 3".to_string(),
            recurrence_id: None,
            tags: None,
        });

        let mut expenses: Vec<Expense> = Vec::new();
        expenses.push(Expense {
            id: 1,
            active: true,
            name: "Expense 1".to_string(),
            recurrence_id: None,
            tags: None,
        });
        expenses.push(Expense {
            id: 2,
            active: false,
            name: "Expense 2".to_string(),
            recurrence_id: None,
            tags: None,
        });
        expenses.push(Expense {
            id: 3,
            active: true,
            name: "Expense 2".to_string(),
            recurrence_id: None,
            tags: None,
        });
        SeedData { incomes, expenses }
    }
}
