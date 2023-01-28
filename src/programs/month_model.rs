use crate::calendar::day::{Day, DayStore};
use crate::calendar::month::{Month, MonthKey};
use crate::composite::account_summary::AccountSummary;
use crate::composite::payment_composite::PaymentCompositeStore;
use crate::composite::payment_display::{PaymentDisplay, PaymentDisplayStore};
use crate::composite::payment_event::PaymentEvent;
use crate::composite::payment_received_composite::PaymentReceivedCompositeStore;
use crate::composite::payment_summary::PaymentSummary;
use crate::composite::payment_summary::PaymentSummaryStore;
use crate::schema::expense::Expense;
use crate::storage::store::Store;
use crate::traits::csv_store::CsvStore;
use chrono::NaiveDate;
use rust_decimal::Decimal;
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
        year: i32,
        month_key: MonthKey,
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
            key: month_key,
            year,
            month: Month::new(year, month_key),
            output_results,
            path_in: data_in,
            path_out: data_out,
        }
    }

    // Model Payments and PaymentsReceived occuring at specific times throughout the specified month
    // TODO: still early testing
    pub fn run(
        &mut self,
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

        self.record_payment_events_in_month();

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

    // TODO: PaymentEvent as JSONRecord
    pub fn record_payment_events_in_month(&mut self) {
        let payment_events = self.payment_events();
        for payment_event in payment_events.iter() {
            for (_id, day) in self.month.days.iter_mut() {
                if payment_event.4.date() == day.date {
                    day.add_payment_event(payment_event.clone());
                }
            }
        }
    }

    pub fn payment_events(&self) -> Vec<PaymentEvent> {
        // TODO: enum for payment, payment_received
        vec![
            PaymentEvent(
                "payment_received",
                "Spaceman".to_string(),
                "Big Bank".to_string(),
                Decimal::new(10000, 0),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 6)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
            ),
            PaymentEvent(
                "payment_received",
                "Cowboy".to_string(),
                "Credit Union".to_string(),
                Decimal::new(10000, 0),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 20)
                    .unwrap()
                    .and_hms_opt(12, 00, 00)
                    .unwrap(),
            ),
            PaymentEvent(
                "payment",
                "Mortgage".to_string(),
                "Big Bank".to_string(),
                Decimal::new(3100, 0),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 10)
                    .unwrap()
                    .and_hms_opt(15, 00, 00)
                    .unwrap(),
            ),
            PaymentEvent(
                "payment",
                "Natural Gas".to_string(),
                "Credit Union".to_string(),
                Decimal::new(125, 0),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 6)
                    .unwrap()
                    .and_hms_opt(15, 00, 00)
                    .unwrap(),
            ),
            PaymentEvent(
                "payment",
                "Natural Gas".to_string(),
                "Credit Union".to_string(),
                Decimal::new(125, 0),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 6)
                    .unwrap()
                    .and_hms_opt(15, 00, 00)
                    .unwrap(),
            ),
            PaymentEvent(
                "payment",
                "Cable".to_string(),
                "Big Bank".to_string(),
                Decimal::new(80, 0),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 10)
                    .unwrap()
                    .and_hms_opt(15, 00, 00)
                    .unwrap(),
            ),
            PaymentEvent(
                "payment",
                "Garbage/Recycling".to_string(),
                "Credit Union".to_string(),
                Decimal::new(60, 0),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 2)
                    .unwrap()
                    .and_hms_opt(15, 00, 00)
                    .unwrap(),
            ),
            PaymentEvent(
                "payment",
                "Groceries".to_string(),
                "Big Bank".to_string(),
                Decimal::new(250, 0),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 7)
                    .unwrap()
                    .and_hms_opt(15, 00, 00)
                    .unwrap(),
            ),
            PaymentEvent(
                "payment",
                "Groceries".to_string(),
                "Big Bank".to_string(),
                Decimal::new(250, 0),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 14)
                    .unwrap()
                    .and_hms_opt(15, 00, 00)
                    .unwrap(),
            ),
            PaymentEvent(
                "payment",
                "Groceries".to_string(),
                "Credit Union".to_string(),
                Decimal::new(250, 0),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 21)
                    .unwrap()
                    .and_hms_opt(15, 00, 00)
                    .unwrap(),
            ),
            PaymentEvent(
                "payment",
                "Groceries".to_string(),
                "Big Bank".to_string(),
                Decimal::new(250, 0),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 28)
                    .unwrap()
                    .and_hms_opt(15, 00, 00)
                    .unwrap(),
            ),
            PaymentEvent(
                "payment",
                "Dog Food".to_string(),
                "Big Bank".to_string(),
                Decimal::new(4699, 2),
                NaiveDate::from_ymd_opt(self.year, Month::id(self.key), 17)
                    .unwrap()
                    .and_hms_opt(13, 00, 00)
                    .unwrap(),
            ),
        ]
    }
}
