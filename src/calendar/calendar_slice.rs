use crate::calendar::month::Month;
use crate::calendar::month_key::MonthKey as MK;
use crate::calendar::year_month::YearMonth as YM;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct CalendarSlice {
    pub start: YM,
    pub end: YM,
}

type CalendarSliceResult = Result<CalendarSlice, Box<dyn Error>>;
type CalendarSliceValidation = Result<(), Box<dyn Error>>;

impl CalendarSlice {
    pub fn new(start: YM, end: YM) -> CalendarSliceResult {
        let calendar_slice = CalendarSlice { start, end };
        calendar_slice.validate()?;
        Ok(calendar_slice)
    }

    pub fn validate(&self) -> CalendarSliceValidation {
        if self.end.year < self.start.year {
            return Err("End year must come after start year.".into());
        }

        if self.end.year == self.start.year
            && Month::id(self.end.month) < Month::id(self.start.month)
        {
            return Err("End month must come after start month.".into());
        }

        Ok(())
    }

    pub fn months(&self) -> Vec<YM> {
        if self.start.year == self.end.year && self.start.month == self.end.month {
            return vec![self.start];
        }

        let mut months: Vec<YM> = vec![];
        let mut curr = self.start;
        while self.in_slice_bounds(curr) {
            months.push(curr);
            if curr.month == MK::Dec {
                curr.year += 1;
            }
            curr.month = Month::next_month(curr.month);
        }
        months
    }

    pub fn in_slice_bounds(&self, curr: YM) -> bool {
        if curr.year < self.end.year {
            // curr.year <= self.end.year by construction
            return true;
        }

        if Month::id(curr.month) < Month::id(Month::next_month(self.end.month)) {
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
    fn new__validates_start_comes_before_end() {
        let start = YM::new(2023, MK::Feb);
        let mut end = YM::new(2023, MK::Jan);

        if let Ok(_) = CalendarSlice::new(start, end) {
            panic!() // fail test if CalendarSlice::new() did not error out
        }

        end = YM::new(2022, MK::Feb);

        if let Ok(_) = CalendarSlice::new(start, end) {
            panic!() // fail test if CalendarSlice::new() did not error out
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn validate__returns_err_when_end_year_before_start_year() {
        let start = YM::new(2023, MK::Feb);
        let end = YM::new(2022, MK::Jan);

        let calendar_slice = CalendarSlice { start, end };

        if let Ok(_) = calendar_slice.validate() {
            panic!() // fail test if CalendarSlice::new() did not error out
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn validate__returns_err_when_end_year_eq_start_year_and_end_month_before_start_month() {
        let start = YM::new(2023, MK::Feb);
        let end = YM::new(2023, MK::Jan);

        let calendar_slice = CalendarSlice { start, end };

        if let Ok(_) = calendar_slice.validate() {
            panic!() // fail test if CalendarSlice::new() did not error out
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn new__validates_start_month_before_end_month_when_same_year() {
        // CalendarSlice::new(2023, MK::Jan, 2022, MK::Dec);

        // TODO: assert errors
        //assert_eq!(*Box::leak(res), Err("End year must come after start year.".into()));
    }

    #[test]
    #[allow(non_snake_case)]
    fn months__returns_array_of_month_keys_in_cyclic_chrono_order() {
        let months = CalendarSlice::new(YM::new(2023, MK::Jun), YM::new(2024, MK::Mar))
            .unwrap()
            .months();

        assert_eq!(10, months.len());
        assert_eq!(MK::Jun, months[0].month);
        assert_eq!(MK::Jul, months[1].month);
        assert_eq!(MK::Aug, months[2].month);
        assert_eq!(MK::Sep, months[3].month);
        assert_eq!(MK::Oct, months[4].month);
        assert_eq!(MK::Nov, months[5].month);
        assert_eq!(MK::Dec, months[6].month);
        assert_eq!(MK::Jan, months[7].month);
        assert_eq!(MK::Feb, months[8].month);
        assert_eq!(MK::Mar, months[9].month);
    }
}
