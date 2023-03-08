use crate::calendar::day::DayStore;
use crate::calendar::month_key::MonthKey as MK;
use crate::calendar::year_month::YearMonth as YM;
use crate::composite::payment_display::{PaymentDisplay, PaymentDisplayStore};
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Month {
    pub key: MK,
    pub days: DayStore,
    pub year: i32,
}

impl Month {
    pub fn new(year_month: YM) -> Month {
        Month {
            key: year_month.month,
            days: DayStore::new(),
            year: year_month.year,
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

    pub fn expense_ids(&self) -> Vec<usize> {
        let mut expense_ids: BTreeMap<usize, bool> = BTreeMap::new();
        for (_id, day) in self.days.iter() {
            for (_p_id, payment) in day.payments.iter() {
                expense_ids
                    .entry(payment.expense_id.unwrap())
                    .or_insert(true);
            }
        }

        expense_ids.keys().cloned().collect()
    }

    pub fn income_ids(&self) -> Vec<usize> {
        let mut income_ids: BTreeMap<usize, bool> = BTreeMap::new();
        for (_id, day) in self.days.iter() {
            for (_pr_id, payment_rec) in day.payments_received.iter() {
                income_ids
                    .entry(payment_rec.income_id.unwrap())
                    .or_insert(true);
            }
        }

        income_ids.keys().cloned().collect()
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

    pub fn id(month: MK) -> u32 {
        // u32 expected by NaiveDate
        match month {
            MK::Jan => 1,
            MK::Feb => 2,
            MK::Mar => 3,
            MK::Apr => 4,
            MK::May => 5,
            MK::Jun => 6,
            MK::Jul => 7,
            MK::Aug => 8,
            MK::Sep => 9,
            MK::Oct => 10,
            MK::Nov => 11,
            MK::Dec => 12,
            MK::None => 0,
        }
    }

    pub fn key_from_id(id: u32) -> MK {
        // u32 expected by NaiveDate
        match id {
            1 => MK::Jan,
            2 => MK::Feb,
            3 => MK::Mar,
            4 => MK::Apr,
            5 => MK::May,
            6 => MK::Jun,
            7 => MK::Jul,
            8 => MK::Aug,
            9 => MK::Sep,
            10 => MK::Oct,
            11 => MK::Nov,
            12 => MK::Dec,
            _ => MK::None,
        }
    }

    pub fn length(month: MK) -> u32 {
        // u32 expected by NaiveDate
        match month {
            MK::Jan => 31,
            MK::Feb => 28,
            MK::Mar => 31,
            MK::Apr => 30,
            MK::May => 31,
            MK::Jun => 30,
            MK::Jul => 31,
            MK::Aug => 31,
            MK::Sep => 30,
            MK::Oct => 31,
            MK::Nov => 30,
            MK::Dec => 31,
            MK::None => 0,
        }
    }

    pub fn next_month(month: MK) -> MK {
        // u32 expected by NaiveDate
        match month {
            MK::Jan => MK::Feb,
            MK::Feb => MK::Mar,
            MK::Mar => MK::Apr,
            MK::Apr => MK::May,
            MK::May => MK::Jun,
            MK::Jun => MK::Jul,
            MK::Jul => MK::Aug,
            MK::Aug => MK::Sep,
            MK::Sep => MK::Oct,
            MK::Oct => MK::Nov,
            MK::Nov => MK::Dec,
            MK::Dec => MK::Jan,
            MK::None => MK::None,
        }
    }

    #[allow(unused)]
    pub fn name(&self) -> &'static str {
        Month::display_name(self.key)
    }

    #[allow(unused)]
    pub fn display_name(month: MK) -> &'static str {
        match month {
            MK::Jan => "January",
            MK::Feb => "February",
            MK::Mar => "March",
            MK::Apr => "April",
            MK::May => "May",
            MK::Jun => "June",
            MK::Jul => "July",
            MK::Aug => "August",
            MK::Sep => "September",
            MK::Oct => "October",
            MK::Nov => "November",
            MK::Dec => "December",
            MK::None => "Invalid MonthKey",
        }
    }

    #[allow(unused)]
    pub fn display_number(&self) -> &str {
        match self.key {
            MK::Jan => "01",
            MK::Feb => "02",
            MK::Mar => "03",
            MK::Apr => "04",
            MK::May => "05",
            MK::Jun => "06",
            MK::Jul => "07",
            MK::Aug => "08",
            MK::Sep => "09",
            MK::Oct => "10",
            MK::Nov => "11",
            MK::Dec => "12",
            MK::None => "Invalid MonthKey",
        }
    }
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

        let res = Month::new(YM::new(2023, MK::Jan));
        assert_eq!("January", res.name());
    }
}
