use crate::calendar::day::{Day, DayStore};
use crate::calendar::month::{Month, MonthKey};
use crate::calendar::year_month::YearMonth;
use crate::composite::account_summary::AccountSummary;
use crate::composite::payment_composite::PaymentCompositeStore;
use crate::composite::payment_display::{PaymentDisplay, PaymentDisplayStore};
use crate::composite::payment_event::PaymentEventStore;
use crate::composite::payment_received_composite::PaymentReceivedCompositeStore;
use crate::composite::payment_summary::PaymentSummary;
use crate::composite::payment_summary::PaymentSummaryStore;
use crate::schema::expense::Expense;
use crate::storage::store::Store;
use crate::traits::csv_store::CsvStore;
use chrono::NaiveDate;
use std::collections::BTreeMap;
use std::error::Error;

pub struct MonthModel {
    year: i32,
    key: MonthKey,
    month: Month,
    output_results: bool,
    #[allow(unused)]
    path_in: &'static str,
    #[allow(unused)]
    path_out: &'static str,
}

impl MonthModel {
    pub fn new(
        year_month: YearMonth,
        output_results: bool,
        path_in: Option<&'static str>,
        path_out: Option<&'static str>,
    ) -> MonthModel {
        let data_in = match path_in {
            None => "data/init/",
            Some(path) => path,
        };

        let data_out = match path_out {
            None => "data/",
            Some(path) => path,
        };

        MonthModel {
            key: year_month.1,
            year: year_month.0,
            month: Month::new(year_month),
            output_results,
            path_in: data_in,
            path_out: data_out,
        }
    }

    // Model Payments and PaymentsReceived occuring at specific times throughout the specified month
    pub fn run(
        &mut self,
        payment_events: &PaymentEventStore,
        store_ext: Option<&mut Store>,
        dir: Option<&'static str>,
    ) -> Result<(), Box<dyn Error>> {
        let path: Option<&str> = match dir {
            None => Some("data/init/"),
            Some(directory) => Some(directory),
        };

        let mut self_store = Store::new();
        let store = match store_ext {
            Some(passed_in) => passed_in,
            None => {
                self_store.init(path)?;
                &mut self_store
            }
        };

        self.month = Month {
            key: self.key,
            days: MonthModel::construct_days(self.year, self.key),
            year: self.year,
        };

        self.record_payment_events_in_month(payment_events);

        // iterate through the days and execute payments in order
        // each payment event mutates store
        for (_id, day) in self.month.days.iter_mut() {
            // iter sorted by key thx to btree_map
            day.execute_payments_in_order(store)?;
        }

        if self.output_results {
            let account_summary_store = AccountSummary::by_id(1, store);
            AccountSummary::write_to_csv(&account_summary_store, "data/account_1_summary.csv")?;

            let expense_summary = MonthModel::construct_payment_summary(store);
            PaymentSummary::write_to_csv(&expense_summary, "data/expense_summary.csv")?;

            let all_payment_disp_store: PaymentDisplayStore = self.month.all_payments_display();
            PaymentDisplay::write_to_csv(&all_payment_disp_store, "data/all_payments.csv")?;

            let all_payment_rec_disp_store: PaymentDisplayStore =
                self.month.all_payments_received_display();
            PaymentDisplay::write_to_csv(
                &all_payment_rec_disp_store,
                "data/all_payments_received.csv",
            )?;

            store.write_to_csv(None)?;
        }

        Ok(())
    }

    pub fn construct_payment_summary(store: &mut Store) -> PaymentSummaryStore {
        let mut payment_summary_store = PaymentSummaryStore::new();
        let expense_ids: Vec<usize> = store.expenses.keys().cloned().collect();
        for expense_id in expense_ids {
            // sorted expense ids
            payment_summary_store
                .entry(expense_id)
                .or_insert(PaymentSummary {
                    id: Some(expense_id),
                    name: Expense::name_by_id(expense_id, store).to_string(),
                    total: Expense::total_by_id(expense_id, store),
                });
        }
        payment_summary_store
    }

    // TODO: leap years
    pub fn construct_days(year: i32, month: MonthKey) -> DayStore {
        let length: u32 = Month::length(month);
        let month_id: u32 = Month::id(month);
        let mut days: DayStore = BTreeMap::new();

        for date in 1..length + 1 {
            let id = usize::try_from(date).unwrap();
            days.entry(id).or_insert(Day {
                id: Some(id),
                payments: PaymentCompositeStore::new(),
                payments_received: PaymentReceivedCompositeStore::new(),
                date: NaiveDate::from_ymd_opt(year, month_id, date).unwrap(),
            });
        }

        days
    }

    pub fn record_payment_events_in_month(&mut self, payment_events: &PaymentEventStore) {
        for (_id, payment_event) in payment_events.iter() {
            for (_id, day) in self.month.days.iter_mut() {
                // inner loop is O(1)
                if payment_event.completed_at.date() == day.date {
                    day.add_payment_event(payment_event);
                }
            }
        }
    }
}

#[cfg(test)]
mod month_model_spec {
    use super::*;
    // use crate::test::spec::Spec;
    // use chrono::NaiveDate;

    pub fn model() -> MonthModel {
        MonthModel::new(
            YearMonth(2023, MonthKey::Feb),
            false,
            Some("src/test/data/init"),
            None,
        )
    }

    //TODO: specs
    #[test]
    #[allow(non_snake_case)]
    fn record_payment_events_in_month__adds_payment_composites_to_days() {
        let _model = model();
    }

    #[test]
    #[allow(non_snake_case)]
    fn construct_days__returns_daystore_of_length_equal_to_days_in_month() {
        let jan_days = MonthModel::construct_days(2023, MonthKey::Jan);
        assert_eq!(31, jan_days.len());
        let feb_days = MonthModel::construct_days(2023, MonthKey::Feb);
        assert_eq!(28, feb_days.len());
        let mar_days = MonthModel::construct_days(2023, MonthKey::Mar);
        assert_eq!(31, mar_days.len());
        let apr_days = MonthModel::construct_days(2023, MonthKey::Apr);
        assert_eq!(30, apr_days.len());
        let may_days = MonthModel::construct_days(2023, MonthKey::May);
        assert_eq!(31, may_days.len());
        let jun_days = MonthModel::construct_days(2023, MonthKey::Jun);
        assert_eq!(30, jun_days.len());
        let jul_days = MonthModel::construct_days(2023, MonthKey::Jul);
        assert_eq!(31, jul_days.len());
        let aug_days = MonthModel::construct_days(2023, MonthKey::Aug);
        assert_eq!(31, aug_days.len());
        let sep_days = MonthModel::construct_days(2023, MonthKey::Sep);
        assert_eq!(30, sep_days.len());
        let oct_days = MonthModel::construct_days(2023, MonthKey::Oct);
        assert_eq!(31, oct_days.len());
        let nov_days = MonthModel::construct_days(2023, MonthKey::Nov);
        assert_eq!(30, nov_days.len());
        let dec_days = MonthModel::construct_days(2023, MonthKey::Dec);
        assert_eq!(31, dec_days.len());
    }
}
