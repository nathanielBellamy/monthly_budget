use crate::calendar::day::Day;
use chrono::Local;

pub struct Month {
    pub key: MonthKey,
    pub days: Vec<Day>,
}

impl Month {
    pub fn add_day(&mut self, day: Day) -> () {
        self.days.push(day);
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

    #[test]
    #[allow(non_snake_case)]
    fn add_day__adds_day_to_self_days() {
        let mut month = Month {
            key: MonthKey::Jan,
            days: vec![],
        };
        let day = Day {
            payments: vec![],
            payments_received: vec![],
            date: Local::now(),
        };

        assert_eq!(0, month.days.len());

        month.add_day(day);

        assert_eq!(1, month.days.len());
    }
}
