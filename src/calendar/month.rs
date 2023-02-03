use crate::calendar::day::DayStore;
use crate::calendar::year_month::YearMonth;
use crate::composite::payment_display::{PaymentDisplay, PaymentDisplayStore};
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;

#[derive(Debug)]
pub struct Month {
    pub key: MonthKey,
    pub days: DayStore,
    pub year: i32,
}

impl Month {
    #[allow(unused)]
    pub fn new(year_month: YearMonth) -> Month {
        Month {
            key: year_month.1,
            days: DayStore::new(),
            year: year_month.0,
        }
    }

    pub fn all_payments_display(&mut self) -> PaymentDisplayStore {
        let mut all_pd: Vec<PaymentDisplay> = vec![];
        for (_id, day) in self.days.iter_mut() {
            for (_id, payment) in day.payments.iter_mut() {
                all_pd.push(payment.display());
            }
        }
        all_pd.sort_by(|a, b| a.completed_at.partial_cmp(&b.completed_at).unwrap());

        let mut store = PaymentDisplayStore::new();
        for pd in all_pd.iter() {
            let mut new_pd = pd.clone_record();
            new_pd.id = None; // clear id tied to day, will be set in chrono order for month
            PaymentDisplay::save_to_store(new_pd, &mut store);
        }

        store
    }

    pub fn all_payments_received_display(&mut self) -> PaymentDisplayStore {
        let mut all_pd: Vec<PaymentDisplay> = vec![];
        for (_id, day) in self.days.iter_mut() {
            for (_id, payment_rec) in day.payments_received.iter_mut() {
                all_pd.push(payment_rec.display());
            }
        }
        all_pd.sort_by(|a, b| a.completed_at.partial_cmp(&b.completed_at).unwrap());

        let mut store = PaymentDisplayStore::new();
        for pd in all_pd.iter() {
            let mut new_pd = pd.clone_record();
            new_pd.id = None; // clear id tied to day, will be set in chrono order for month
            PaymentDisplay::save_to_store(new_pd, &mut store);
        }

        store
    }

    pub fn id(month: MonthKey) -> u32 {
        // u32 expected by NaiveDate
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
            MonthKey::None => 0,
        }
    }

    pub fn key_from_id(id: u32) -> MonthKey {
        // u32 expected by NaiveDate
        match id {
            1 => MonthKey::Jan,
            2 => MonthKey::Feb,
            3 => MonthKey::Mar,
            4 => MonthKey::Apr,
            5 => MonthKey::May,
            6 => MonthKey::Jun,
            7 => MonthKey::Jul,
            8 => MonthKey::Aug,
            9 => MonthKey::Sep,
            10 => MonthKey::Oct,
            11 => MonthKey::Nov,
            12 => MonthKey::Dec,
            _ => MonthKey::None,
        }
    }

    pub fn length(month: MonthKey) -> u32 {
        // u32 expected by NaiveDate
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
            MonthKey::None => 0,
        }
    }

    pub fn next_month(month: MonthKey) -> MonthKey {
        // u32 expected by NaiveDate
        match month {
            MonthKey::Jan => MonthKey::Feb,
            MonthKey::Feb => MonthKey::Mar,
            MonthKey::Mar => MonthKey::Apr,
            MonthKey::Apr => MonthKey::May,
            MonthKey::May => MonthKey::Jun,
            MonthKey::Jun => MonthKey::Jul,
            MonthKey::Jul => MonthKey::Aug,
            MonthKey::Aug => MonthKey::Sep,
            MonthKey::Sep => MonthKey::Oct,
            MonthKey::Oct => MonthKey::Nov,
            MonthKey::Nov => MonthKey::Dec,
            MonthKey::Dec => MonthKey::Jan,
            MonthKey::None => MonthKey::None,
        }
    }

    #[allow(unused)]
    pub fn name(&self) -> &'static str {
        Month::display_name(self.key)
    }

    #[allow(unused)]
    pub fn display_name(month: MonthKey) -> &'static str {
        match month {
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
            MonthKey::None => "Invalid MonthKey",
        }
    }

    #[allow(unused)]
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
            MonthKey::None => "Invalid MonthKey",
        }
    }
}

#[derive(Eq, PartialOrd, Ord, Clone, Copy, Debug, PartialEq)]
#[allow(unused)]
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
    None,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::store::Store;
    use crate::test::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn new__returns_month() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let res = Month::new(YearMonth(2023, MonthKey::Jan));
        assert_eq!("January", res.name());
    }
}
