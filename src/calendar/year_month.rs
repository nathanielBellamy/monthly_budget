use crate::calendar::month::MonthKey;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
// derived traits allow struct to be used as keys in BTreeMap
pub struct YearMonth(pub i32, pub MonthKey);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn year_month__implements_eq() {
        let year_month_1 = YearMonth(2023_i32, MonthKey::Feb);
        let year_month_2 = YearMonth(2023_i32, MonthKey::Feb);

        assert_eq!(year_month_1, year_month_2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn year_month__implements_partial_ord() {
        let year_month_1 = YearMonth(2022_i32, MonthKey::Feb);
        let year_month_2 = YearMonth(2023_i32, MonthKey::Feb);
        let year_month_3 = YearMonth(2023_i32, MonthKey::Mar);

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
