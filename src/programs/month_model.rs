use crate::composite::payment_display::{PaymentDisplay, PaymentDisplayStore};
use std::collections::BTreeMap;
use chrono::{NaiveDate};
use std::error::Error;
use crate::calendar::month::{MonthKey, Month};
use crate::calendar::day::{Day, DayStore};
use crate::composite::payment_composite::{PaymentComposite, PaymentCompositeStore};
use crate::composite::payment_received_composite::{PaymentReceivedComposite, PaymentReceivedCompositeStore};
use crate::store::store::Store;
use std::collections::btree_map::Entry;
use crate::traits::csv_store::CsvStore;
pub struct MonthModel {
  key: MonthKey,
}

impl MonthModel {
  pub fn new(month: MonthKey) -> MonthModel {
    MonthModel {
      key: month
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

    // Set-up payments
    // TODO: import this data from a JSON or Csv

    // Pay mortgage at 5:30 pm on the 1st
    if let Entry::Occupied(mut day) = month.days.entry(1){
      day.get_mut().add_payment(
        PaymentComposite {
          id: None,
          account_balance_id: None,
          account_id: None,
          account_name: String::from("piggybank"),
          amount_id: None,
          amount_standard: 1000.0,
          payment_id: None,
          payment_completed_at: NaiveDate::from_ymd_opt(2023, Month::id(self.key), 1).unwrap()
                                          .and_hms_opt(17, 30, 00).unwrap(),
          expense_id: None,
          expense_name: "mortgage".to_string(),
          prev_balance: None,
          ending_balance: None,
      });
    }

    // Pay electric bill at 6:00 pm on the 1st
    if let Entry::Occupied(mut day) = month.days.entry(1){
      day.get_mut().add_payment(
        PaymentComposite {
          id: None,
          account_balance_id: None,
          account_id: None,
          account_name: String::from("swearjar"),
          amount_id: None,
          amount_standard: 100.0,
          payment_id: None,
          payment_completed_at: NaiveDate::from_ymd_opt(2023, Month::id(self.key), 1).unwrap()
                                          .and_hms_opt(18, 00, 00).unwrap(),
          expense_id: None,
          expense_name: "electric".to_string(),
          prev_balance: None,
          ending_balance: None,
      });
    }

    // Get paid at 3:00pm on the 3rd and 20th
    let pay_dates: [usize; 2] = [3, 20];
    for date in pay_dates.iter() {
      if let Entry::Occupied(mut day) = month.days.entry(*date){
        day.get_mut().add_payment_received(
          PaymentReceivedComposite {
            id: None,
            account_balance_id: None,
            account_id: None,
            account_name: String::from("piggybank"),
            amount_id: None,
            amount_standard: 10000.00,
            payment_received_id: None,
            payment_received_completed_at: NaiveDate::from_ymd_opt(2023, Month::id(self.key), *date as u32).unwrap()
                                            .and_hms_opt(15, 00, 00).unwrap(),
            income_id: None,
            income_name: "spaceman".to_string(),
            prev_balance: None,
            ending_balance: None,

        });
      }
    }

    // groceirs on the 7th, 14th, 21st, 28th at 4:00pm
    let grocery_dates: [usize; 4] = [7, 14, 21, 28];
    for date in grocery_dates.iter() {
      if let Entry::Occupied(mut day) = month.days.entry(*date){
        day.get_mut().add_payment(
          PaymentComposite {
            id: None,
            account_balance_id: None,
            account_id: None,
            account_name: String::from("piggybank"),
            amount_id: None,
            amount_standard: 250.00,
            payment_id: None,
            payment_completed_at: NaiveDate::from_ymd_opt(2023, Month::id(self.key), *date as u32).unwrap()
                                            .and_hms_opt(16, 00, 00).unwrap(),
            expense_id: None,
            expense_name: "grocery".to_string(),
            prev_balance: None,
            ending_balance: None,
        });
      }
    }

    // take the dog to the vet on the 18th at 1:30pm
    if let Entry::Occupied(mut day) = month.days.entry(18){
      day.get_mut().add_payment(
        PaymentComposite {
          id: None,
          account_balance_id: None,
          account_id: None,
          account_name: String::from("swearjar"),
          amount_id: None,
          amount_standard: 500.0,
          payment_id: None,
          payment_completed_at: NaiveDate::from_ymd_opt(2023, Month::id(self.key), 18).unwrap()
                                          .and_hms_opt(13, 30, 00).unwrap(),
          expense_id: None,
          expense_name: "vet".to_string(),
          prev_balance: None,
          ending_balance: None,
      });
    }

    // swear when stubbing toe at 4:26 am on the 6th, 13th, 26th
    // TODO: handle "transfers"
    let curse_dates: [usize; 3] = [6, 13, 26];
    for date in curse_dates.iter() {
      if let Entry::Occupied(mut day) = month.days.entry(*date){
        day.get_mut().add_payment_received(
          PaymentReceivedComposite {
            id: None,
            account_balance_id: None,
            account_id: None,
            account_name: String::from("swearjar"),
            amount_id: None,
            amount_standard: 1.00,
            payment_received_id: None,
            payment_received_completed_at: NaiveDate::from_ymd_opt(2023, Month::id(self.key), *date as u32).unwrap()
                                            .and_hms_opt(4, 26, 00).unwrap(),
            income_id: None,
            income_name: "curse_words".to_string(),
            prev_balance: None,
            ending_balance: None,
        });
      }
    }

    // iterate through the days and execute payments in order
    // each payment event mutates store
    for (_id, day) in month.days.iter_mut() { // iter sorted by key thx to btree_map
        day.execute_payments_in_order(&mut store)?;
    }

    let mut all_payment_disp_store: PaymentDisplayStore = month.all_payments_display();
    PaymentDisplay::write_to_csv(&mut all_payment_disp_store, "data/all_payments.csv")?;

    let mut all_payment_rec_disp_store: PaymentDisplayStore = month.all_payments_received_display();
    PaymentDisplay::write_to_csv(&mut all_payment_rec_disp_store, "data/all_payments_received.csv")?;

    store.write_to_csv(None)?;

    Ok(())
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

}

