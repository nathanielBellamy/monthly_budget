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
            res += income.amount;
        }
        res
    }

    pub fn gross_expense(&self) -> usize {
        let mut res: usize = 0;
        for expense in self.expenses.iter() {
            res += expense.amount;
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

pub struct Income<'a> {
    pub active: bool,
    pub amount: usize,
    pub name: &'a str,
    pub recurring: bool,
    /*
      start_month
      end_month
      tags - add tags to be able to run different scenarios
    */
}

pub struct Amount {
    // use to over/under estimate
    pub low: usize,
    pub high: usize,
    pub average: usize,
}

impl Amount {
    pub fn randomize() -> usize {
        // random number between low and high
        0
    }
}

pub struct Expense<'a> {
    pub active: bool,
    pub amount: usize,
    pub name: &'a str,
    pub recurring: bool,
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
