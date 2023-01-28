use crate::calendar::month::{Month, MonthKey};
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct CalendarSlice {
    pub start_year: i32, // type expected by NaiveDate
    pub start_month: MonthKey,
    pub end_year: i32,
    pub end_month: MonthKey,
}

type CalendarSliceResult = Result<CalendarSlice, Box<dyn Error>>;

impl CalendarSlice {
    pub fn new(
        start_year: i32,
        start_month: MonthKey,
        end_year: i32,
        end_month: MonthKey,
    ) -> CalendarSliceResult {
        if end_year < start_year {
            return Err("End year must come after start year.".into());
        }

        if end_year == start_year && Month::id(end_month) < Month::id(start_month) {
            return Err("End month must come after start month.".into());
        }

        Ok(CalendarSlice {
            start_year,
            start_month,
            end_year,
            end_month,
        })
    }

    pub fn months(&self) -> Vec<(i32, MonthKey)> {
        //(year, month)
        if self.start_month == self.end_month && self.start_year == self.end_year {
            return vec![(self.start_year, self.start_month)];
        }

        let mut months: Vec<(i32, MonthKey)> = vec![];
        let mut curr_year = self.start_year;
        let mut curr_month = self.start_month;
        while self.in_slice_bounds(curr_year, curr_month) {
            months.push((curr_year, curr_month));
            if curr_month == MonthKey::Dec {
                curr_year += 1;
            }
            curr_month = Month::next_month(curr_month);
        }
        months
    }

    pub fn in_slice_bounds(&self, curr_year: i32, curr_month: MonthKey) -> bool {
        if curr_year < self.end_year {
            return true;
        }

        if Month::id(curr_month) < Month::id(Month::next_month(self.end_month)) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn new__validates_start_year_before_end_year() {
        // CalendarSlice::new(2023, MonthKey::Jan, 2022, MonthKey::Dec);

        // TODO: assert errors
        //assert_eq!(*Box::leak(res), Err("End year must come after start year.".into()));
    }

    #[test]
    #[allow(non_snake_case)]
    fn new__validates_start_month_before_end_month_when_same_year() {
        // CalendarSlice::new(2023, MonthKey::Jan, 2022, MonthKey::Dec);

        // TODO: assert errors
        //assert_eq!(*Box::leak(res), Err("End year must come after start year.".into()));
    }

    #[test]
    #[allow(non_snake_case)]
    fn months__returns_array_of_month_keys_in_cyclic_chrono_order() {
        let months = CalendarSlice::new(2023, MonthKey::Jun, 2024, MonthKey::Mar)
            .unwrap()
            .months();

        assert_eq!(10, months.len());
        assert_eq!(MonthKey::Jun, months[0].1);
        assert_eq!(MonthKey::Jul, months[1].1);
        assert_eq!(MonthKey::Aug, months[2].1);
        assert_eq!(MonthKey::Sep, months[3].1);
        assert_eq!(MonthKey::Oct, months[4].1);
        assert_eq!(MonthKey::Nov, months[5].1);
        assert_eq!(MonthKey::Dec, months[6].1);
        assert_eq!(MonthKey::Jan, months[7].1);
        assert_eq!(MonthKey::Feb, months[8].1);
        assert_eq!(MonthKey::Mar, months[9].1);
    }
}
