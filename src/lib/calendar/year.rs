use crate::calendar::month::{Month, MonthKey};
use crate::lib::money::expense::Expense;
use crate::lib::money::income::Income;

pub struct Year<'a> {
    pub id: u16,
    jan: Month<'a>,
    feb: Month<'a>,
    mar: Month<'a>,
    apr: Month<'a>,
    may: Month<'a>,
    jun: Month<'a>,
    jul: Month<'a>,
    aug: Month<'a>,
    sep: Month<'a>,
    oct: Month<'a>,
    nov: Month<'a>,
    dec: Month<'a>,
}

impl<'a> Year<'_> {
    pub fn month_array(&self) -> [&Month; 12] {
        [
            &self.jan, &self.feb, &self.mar, &self.apr, &self.may, &self.jun, &self.jul, &self.aug,
            &self.sep, &self.oct, &self.nov, &self.dec,
        ]
    }

    pub fn gross_income(&self) -> usize {
        let mut res: usize = 0;
        for month in self.month_array().iter() {
            res += month.gross_income()
        }
        res
    }

    pub fn gross_expense(&self) -> usize {
        let mut res: usize = 0;
        for month in self.month_array().iter() {
            res += month.gross_expense();
        }
        res
    }

    pub fn net_income(&self) -> usize {
        self.gross_income() - self.gross_expense()
    }
}

pub fn year_2023<'a>() -> Year<'a> {
    Year {
        id: 2023,
        jan: Month {
            id: 1,
            key: MonthKey::Jan,
            budget: 500_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 72_000_000,
        },
        feb: Month {
            id: 2,
            key: MonthKey::Feb,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        mar: Month {
            id: 3,
            key: MonthKey::Mar,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        apr: Month {
            id: 4,
            key: MonthKey::Apr,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        may: Month {
            id: 5,
            key: MonthKey::May,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        jun: Month {
            id: 6,
            key: MonthKey::Jun,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        jul: Month {
            id: 7,
            key: MonthKey::Jul,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        aug: Month {
            id: 8,
            key: MonthKey::Aug,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        sep: Month {
            id: 9,
            key: MonthKey::Sep,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        oct: Month {
            id: 10,
            key: MonthKey::Oct,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        nov: Month {
            id: 11,
            key: MonthKey::Nov,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        dec: Month {
            id: 12,
            key: MonthKey::Dec,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
    }
}

pub fn incomes<'a>() -> Vec<Income<'a>> {
    let mut incomes: Vec<Income> = Vec::new();

    incomes.push(Income {
        id: 1,
        active: true,
        amount: 600_000,
        name: "koho",
        recurrence_id: None,
        tags: None,
    });
    incomes.push(Income {
        id: 2,
        active: true,
        amount: 180_000,
        name: "north_clackamas",
        recurrence_id: None,
        tags: None,
    });
    incomes
}

pub fn expenses<'a>() -> Vec<Expense<'a>> {
    let mut expenses: Vec<Expense> = Vec::new();

    expenses.push(Expense {
        id: 1,
        active: true,
        amount: 300_000,
        name: "mortgage",
        recurrence_id: None,
        tags: None,
    });
    expenses.push(Expense {
        id: 2,
        active: true,
        amount: 100_000,
        name: "bills",
        recurrence_id: None,
        tags: None,
    });

    expenses
}
