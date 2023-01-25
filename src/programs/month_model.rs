use crate::traits::csv_record::CsvRecord;
use crate::composite::payment_summary::PaymentSummary;
use crate::schema::expense::Expense;
use crate::composite::payment_summary::PaymentSummaryStore;
use crate::composite::payment_event::PaymentEvent;
use crate::composite::payment_display::{PaymentDisplay, PaymentDisplayStore};
use std::collections::BTreeMap;
use chrono::{NaiveDate};
use std::error::Error;
use crate::calendar::month::{MonthKey, Month};
use crate::calendar::day::{Day, DayStore};
use crate::composite::payment_composite::{PaymentCompositeStore};
use crate::composite::payment_received_composite::{PaymentReceivedCompositeStore};
use crate::store::store::Store;
use std::collections::btree_map::Entry;
use crate::traits::csv_store::CsvStore;

pub struct MonthModel {
  key: MonthKey,
  path_in: &'static str,
  path_out: &'static str,
}

impl MonthModel {
  pub fn new(month_key: MonthKey, path_in: Option<&'static str>, path_out: Option<&'static str>) -> MonthModel {
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
      path_in: data_in,
      path_out: data_out,
    }
  }

  // Model Payments and PaymentsReceived occuring at specific times throughout the specified month
  // TODO: still early testing
  pub fn run(&self) -> Result<(), Box<dyn Error>> {
    let mut store = Store::new();
    store.init(Some("data/init/"))?;


    let mut month = Month {
      key: self.key,
      days: MonthModel::construct_days(self.key),
    };

    MonthModel::record_payment_events_in_month(&mut month);

    // iterate through the days and execute payments in order
    // each payment event mutates store
    for (_id, day) in month.days.iter_mut() { // iter sorted by key thx to btree_map
        day.execute_payments_in_order(&mut store)?;
    }

    let mut expense_summary = MonthModel::construct_payment_summary(&mut store);
    PaymentSummary::write_to_csv(&mut expense_summary, "data/expense_summary.csv")?;

    let mut all_payment_disp_store: PaymentDisplayStore = month.all_payments_display();
    PaymentDisplay::write_to_csv(&mut all_payment_disp_store, "data/all_payments.csv")?;

    let mut all_payment_rec_disp_store: PaymentDisplayStore = month.all_payments_received_display();
    PaymentDisplay::write_to_csv(&mut all_payment_rec_disp_store, "data/all_payments_received.csv")?;

    store.write_to_csv(None)?;

    Ok(())
  }

  pub fn construct_payment_summary(mut store: &mut Store) -> PaymentSummaryStore {
    let mut payment_summary_store = PaymentSummaryStore::new();
    let expense_ids: Vec<usize> = store.expenses.keys().cloned().collect();
    for expense_id in expense_ids { // sorted expense ids
      payment_summary_store.entry(expense_id).or_insert(
        PaymentSummary{
          id: Some(expense_id),
          name: Expense::name_by_id(expense_id, &mut store).to_string(),
          total: Expense::total_by_id(expense_id, &mut store),
        }
      );
    }
    payment_summary_store
  }

  pub fn construct_days(month: MonthKey) -> DayStore {
    let length: u32 = Month::length(month);
    let month_id: u32 = Month::id(month);
    let mut days: DayStore = BTreeMap::new();

    for date in 1..length+1 {
      let id = usize::try_from(date).unwrap();
      days.entry(id).or_insert(
        Day {
          id: Some(id),
          payments: PaymentCompositeStore::new(),
          payments_received: PaymentReceivedCompositeStore::new(),
          date: NaiveDate::from_ymd_opt(2023, month_id, date).unwrap(),
        }
      );
    }

    days
  }

  // TODO: optomize
  // TODO: PaymentEvent as CSVRecord
  pub fn record_payment_events_in_month<'a>(month: &mut Month) -> (){
    let month_id = Month::id(month.key);
    let month_length = Month::length(month.key);
    let payment_events = MonthModel::payment_events(month);
    // step through days of the month
    for (idx, date) in NaiveDate::from_ymd_opt(2023, month_id, 1).unwrap().iter_days().take(month_length as usize).enumerate() {
      let date_id = idx + 1;
      for (_index, pymt_event) in payment_events.iter().enumerate() {
        if pymt_event.4.date() == date {
          if let Entry::Occupied(mut day) = month.days.entry(date_id) {
            day.get_mut().add_payment_event(pymt_event.clone());
            // payment_events.remove(index); TODO: implement this idea, to reduce unecessary loops
          }
        }
      }
    }
  }

  pub fn payment_events<'a>(month: &mut Month) -> Vec<PaymentEvent> {
        // TODO: ingest this data from somwhere eg. data/month.csv
        // TODO: enum for payment, payment_received
        vec![
          PaymentEvent(
            "payment_received",
            "Spaceman".to_string(),
            "Big Bank".to_string(),
            10000.00,
            NaiveDate::from_ymd_opt(2023, Month::id(month.key), 6).unwrap()
                       .and_hms_opt(12, 00, 00).unwrap()
          ),
          PaymentEvent(
            "payment_received",
            "Cowboy".to_string(),
            "Credit Union".to_string(),
            10000.00,
            NaiveDate::from_ymd_opt(2023, Month::id(month.key), 20).unwrap()
                       .and_hms_opt(12, 00, 00).unwrap()
          ),
          PaymentEvent(
            "payment",
            "Mortgage".to_string(),
            "Big Bank".to_string(),
            3100.0,
            NaiveDate::from_ymd_opt(2023, Month::id(month.key), 10).unwrap()
                       .and_hms_opt(15, 00, 00).unwrap()
          ),
          PaymentEvent(
            "payment",
            "Natural Gas".to_string(),
            "Credit Union".to_string(),
            125.00,
            NaiveDate::from_ymd_opt(2023, Month::id(month.key), 6).unwrap()
                       .and_hms_opt(15, 00, 00).unwrap()
          ),
          PaymentEvent(
            "payment",
            "Cable".to_string(),
            "Big Bank".to_string(),
            80.0,
            NaiveDate::from_ymd_opt(2023, Month::id(month.key), 10).unwrap()
                       .and_hms_opt(15, 00, 00).unwrap()
          ),
          PaymentEvent(
            "payment",
            "Garbage/Recycling".to_string(),
            "Credit Union".to_string(),
            60.0,
            NaiveDate::from_ymd_opt(2023, Month::id(month.key), 2).unwrap()
                       .and_hms_opt(15, 00, 00).unwrap()
          ),
          PaymentEvent(
            "payment",
            "Groceries".to_string(),
            "Big Bank".to_string(),
            250.00,
            NaiveDate::from_ymd_opt(2023, Month::id(month.key), 7).unwrap()
                       .and_hms_opt(15, 00, 00).unwrap()
          ),
          PaymentEvent(
            "payment",
            "Groceries".to_string(),
            "Big Bank".to_string(),
            250.00,
            NaiveDate::from_ymd_opt(2023, Month::id(month.key), 14).unwrap()
                       .and_hms_opt(15, 00, 00).unwrap()
          ),
          PaymentEvent(
            "payment",
            "Groceries".to_string(),
            "Credit Union".to_string(),
            250.00,
            NaiveDate::from_ymd_opt(2023, Month::id(month.key), 21).unwrap()
                       .and_hms_opt(15, 00, 00).unwrap()
          ),
          PaymentEvent(
            "payment",
            "Groceries".to_string(),
            "Big Bank".to_string(),
            250.00,
            NaiveDate::from_ymd_opt(2023, Month::id(month.key), 28).unwrap()
                       .and_hms_opt(15, 00, 00).unwrap()
          ),
          PaymentEvent(
            "payment",
            "Dog Food".to_string(),
            "Big Bank".to_string(),
            46.99,
            NaiveDate::from_ymd_opt(2023, Month::id(month.key), 17 as u32).unwrap()
                       .and_hms_opt(13, 00, 00).unwrap()
          ),
        ]
    }
}
