use crate::lib::money::expense::Expense;
use crate::lib::money::income::Income;

pub struct Month<'a> {
    pub id: u16,
    pub key: MonthKey,
    pub budget: usize,
    pub incomes: Vec<Income<'a>>,
    pub expenses: Vec<Expense<'a>>,
    pub savings_at_start: usize,
}

impl<'a> Month<'_> {
    pub fn gross_income(&self) -> usize {
        let mut res: usize = 0;
        for income in self.incomes.iter() {
            if income.active {
                res += income.amount;
            }
        }
        res
    }

    pub fn gross_expense(&self) -> usize {
        let mut res: usize = 0;
        for expense in self.expenses.iter() {
            if expense.active {
                res += expense.amount;
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

    struct SeedData<'a> {
        incomes: Vec<Income<'a>>,
        expenses: Vec<Expense<'a>>,
    }

    fn seed_data() -> SeedData<'static> {
        let mut incomes: Vec<Income> = Vec::new();
        incomes.push(Income {
            active: true,
            amount: 1_000,
            name: "Income 1",
            recurrence_id: None,
            tags: None,
        });
        incomes.push(Income {
            active: false,
            amount: 2_000,
            name: "Income 2",
            recurrence_id: None,
            tags: None,
        });
        incomes.push(Income {
            active: true,
            amount: 3_000,
            name: "Income 3",
            recurrence_id: None,
            tags: None,
        });

        let mut expenses: Vec<Expense> = Vec::new();
        expenses.push(Expense {
            active: true,
            amount: 1_000,
            name: "Expense 1",
            recurrence_id: None,
            tags: None,
        });
        expenses.push(Expense {
            active: false,
            amount: 1_000,
            name: "Expense 2",
            recurrence_id: None,
            tags: None,
        });
        expenses.push(Expense {
            active: true,
            amount: 3_000,
            name: "Expense 2",
            recurrence_id: None,
            tags: None,
        });
        SeedData { incomes, expenses }
    }

    fn month<'a>() -> Month<'a> {
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
}
