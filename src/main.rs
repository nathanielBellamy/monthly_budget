fn main() {
    let year: Year = year_2023();

    for (idx, month) in year.month_array().iter().enumerate() {
        let month_id = idx + 1;
        println!("{} Starting Savings: {}", month_id, month.savings_at_start);
        println!("{} Budget: {}", month_id, month.budget);
        println!("{} Gross Income: {}", month_id, month.gross_income());
        println!("{} Gross Expense: {}", month_id, month.gross_expense());
        println!("{} Net Income: {}", month_id, month.net_income());
    }
}

struct Year<'a> {
    id: u16,
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
    fn month_array(&self) -> [&Month; 12] {
        [
            &self.jan, &self.feb, &self.mar, &self.apr, &self.may, &self.jun, &self.jul, &self.aug,
            &self.sep, &self.oct, &self.nov, &self.dec,
        ]
    }
}

struct Month<'a> {
    key: MonthKey,
    budget: usize,
    incomes: Vec<Income<'a>>,
    expenses: Vec<Expense<'a>>,
    savings_at_start: usize,
}

impl<'a> Month<'_> {
    fn gross_income(&self) -> usize {
        let mut res: usize = 0;
        for income in self.incomes.iter() {
            res += income.amount;
        }
        res
    }

    fn gross_expense(&self) -> usize {
        let mut res: usize = 0;
        for expense in self.expenses.iter() {
            res += expense.amount;
        }
        res
    }

    fn net_income(&self) -> usize {
        self.gross_income() - self.gross_expense()
    }
}

struct Income<'a> {
    name: &'a str,
    amount: usize,
}

struct Expense<'a> {
    name: &'a str,
    amount: usize,
}

//////

enum MonthKey {
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

fn year_2023<'a>() -> Year<'a> {
    Year {
        id: 2023,
        jan: Month {
            key: MonthKey::Jan,
            budget: 500_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 72_000_000,
        },
        feb: Month {
            key: MonthKey::Feb,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        mar: Month {
            key: MonthKey::Mar,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        apr: Month {
            key: MonthKey::Apr,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        may: Month {
            key: MonthKey::May,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        jun: Month {
            key: MonthKey::Jun,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        jul: Month {
            key: MonthKey::Jul,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        aug: Month {
            key: MonthKey::Aug,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        sep: Month {
            key: MonthKey::Sep,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        oct: Month {
            key: MonthKey::Oct,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        nov: Month {
            key: MonthKey::Nov,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
        dec: Month {
            key: MonthKey::Dec,
            budget: 600_000,
            incomes: incomes(),
            expenses: expenses(),
            savings_at_start: 73_000_000,
        },
    }
}

fn incomes<'a>() -> Vec<Income<'a>> {
    let mut incomes: Vec<Income> = Vec::new();

    incomes.push(Income {
        name: "koho",
        amount: 600_000,
    });
    incomes.push(Income {
        name: "north_clackamas",
        amount: 180_000,
    });
    incomes
}

fn expenses<'a>() -> Vec<Expense<'a>> {
    let mut expenses: Vec<Expense> = Vec::new();

    expenses.push(Expense {
        name: "mortgage",
        amount: 300_000,
    });
    expenses.push(Expense {
        name: "bills",
        amount: 100_000,
    });

    expenses
}
