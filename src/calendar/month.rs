use crate::calendar::day::{Day, DayStore};
use crate::store::store::Store;

#[derive(Debug)]
pub struct Month {
    pub key: MonthKey,
    pub days: DayStore,
}

impl Month {
    pub fn id(month: MonthKey) -> u32 { // u32 expected by NaiveDate
      match month {
        MonthKey::Jan => 1,
        MonthKey::Feb => 2,
        MonthKey::Mar => 3,
        MonthKey::Apr => 4,
        MonthKey::May => 5,
        MonthKey::Jun => 6,
        MonthKey::Jul => 7,
        MonthKey::Aug => 8,
        MonthKey::Sep => 9,
        MonthKey::Oct => 10,
        MonthKey::Nov => 11,
        MonthKey::Dec => 12,
      }
    }

    pub fn length(month: MonthKey) -> u32 { // u32 expected by NaiveDate
      match month {
        MonthKey::Jan => 31,
        MonthKey::Feb => 28,
        MonthKey::Mar => 31,
        MonthKey::Apr => 30,
        MonthKey::May => 31,
        MonthKey::Jun => 30,
        MonthKey::Jul => 31,
        MonthKey::Aug => 31,
        MonthKey::Sep => 30,
        MonthKey::Oct => 31,
        MonthKey::Nov => 30,
        MonthKey::Dec => 31,
      }
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

#[derive(Clone, Copy, Debug)]
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
    use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn test_1() {
        let mut store = Store::new();
        Spec::init(&mut store);

        assert_eq!(2,2);
    }
}
