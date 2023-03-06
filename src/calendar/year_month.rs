use chrono::NaiveDate;

use crate::calendar::month::Month;
use crate::calendar::month_key::MonthKey as MK;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
// derived traits allow struct to be used as keys in BTreeMap
pub struct YearMonth {
    pub year: i32,
    pub month: MK,
}

impl YearMonth {
    pub fn new(year: i32, month: MK) -> YearMonth {
        YearMonth { year, month }
    }

    pub fn start_of_next_month(&self) -> NaiveDate {
        let next_month = Month::next_month(self.month);
        let next_month_id = Month::id(next_month);
        let year = if next_month == MK::Jan { self.year + 1 } else { self.year };
        NaiveDate::from_ymd_opt(year, next_month_id, 01).unwrap()
    }

    pub fn parse(ym: String) -> YearMonth {
        let mut ym_split = ym.split('-');
        let year: i32 = ym_split
            .next()
            .unwrap()
            .parse::<i32>()
            .expect("Invalid Year Entered");
        let month: MK = Month::key_from_id(
            ym_split
                .next()
                .unwrap()
                .parse::<u32>()
                .expect("Invalid Month Entered"),
        );
        YearMonth::new(year, month)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn year_month__implements_eq() {
        let year_month_1 = YearMonth::new(2023_i32, MK::Feb);
        let year_month_2 = YearMonth::new(2023_i32, MK::Feb);

        assert_eq!(year_month_1, year_month_2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn year_month__implements_partial_ord() {
        let year_month_1 = YearMonth::new(2022_i32, MK::Feb);
        let year_month_2 = YearMonth::new(2023_i32, MK::Feb);
        let year_month_3 = YearMonth::new(2023_i32, MK::Mar);

        assert_eq!(true, year_month_1 < year_month_2);
        assert_eq!(true, year_month_1 <= year_month_2);
        assert_eq!(true, year_month_2 < year_month_3);
        assert_eq!(true, year_month_2 <= year_month_3);
        assert_eq!(true, year_month_1 < year_month_3);
        assert_eq!(true, year_month_1 <= year_month_3);
        assert_eq!(true, year_month_1 <= year_month_1);
        assert_eq!(true, year_month_2 <= year_month_2);
        assert_eq!(true, year_month_3 <= year_month_3);
    }
}
