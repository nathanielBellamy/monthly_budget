use std::collections::HashMap;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use std::error::Error;
use crate::calendar::month::{MonthKey, Month};
use crate::calendar::day::{Day, DayStore};

pub struct MonthModel {
  key: MonthKey,
}

impl MonthModel {
  pub fn new(month: MonthKey) -> MonthModel {
    MonthModel {
      key: month
    }
  }

  pub fn run(&self) -> Result<(), Box<dyn Error>> {
    let month = Month {
      key: self.key,
      days: MonthModel::construct_days(self.key),
    };

    println!("{:?}", month);

    Ok(())
  }

  pub fn construct_days(month: MonthKey) -> DayStore {
    let length: u32 = Month::length(month);
    let month_id: u32 = Month::id(month);
    let mut days: DayStore = HashMap::new();

    for date in 1..length+1 {
      let id = usize::try_from(date).unwrap();
      days.entry(id).or_insert(
        Day {
          id: Some(id),
          payments: vec![],
          payments_received: vec![],
          date: NaiveDate::from_ymd_opt(2023, month_id, date).unwrap(),
        }
      );
    }

    days
  }

}


  //   let mut payment_composite_test_1 = PaymentComposite {
  //   account_id: None,
  //   account_name: String::from("piggybank"),
  //   amount_id: None,
  //   amount_standard: 1234.56,
  //   payment_id: None,
  //   payment_completed_at: Utc::now().naive_local(),
  //   expense_id: None,
  //   expense_name: String::from("The Good Stuff"),
  // };

  // payment_composite_test_1.create_payment(&mut main_store);

  // let mut payment_composite_test_2 = PaymentComposite {
  //     account_id: None,
  //     account_name: String::from("new_bank"),
  //     amount_id: None,
  //     amount_standard: 5678.67,
  //     payment_id: None,
  //     payment_completed_at: Utc::now().naive_local(),
  //     expense_id: None,
  //     expense_name: String::from("The Better Stuff"),
  // };

  // payment_composite_test_2.create_payment(&mut main_store);

  // let mut payment_composite_test_3 = PaymentComposite {
  //     account_id: None,
  //     account_name: String::from("credit_union"),
  //     amount_id: None,
  //     amount_standard: 121212.34,
  //     payment_id: None,
  //     payment_completed_at: Utc::now().naive_local(),
  //     expense_id: None,
  //     expense_name: String::from("The Best Stuff"),
  // };

  // payment_composite_test_3.create_payment(&mut main_store);

  // let mut payment_received_composite_test_1 = PaymentReceivedComposite {
  //     account_id: None,
  //     account_name: String::from("credit_union"),
  //     amount_id: None,
  //     amount_standard: 2149055.34,
  //     payment_received_id: None,
  //     payment_received_completed_at: Utc::now().naive_local(),
  //     income_id: None,
  //     income_name: String::from("spaceman"),
  // };

  // payment_received_composite_test_1.create_payment_received(&mut main_store);

  // let mut payment_received_composite_test_2 = PaymentReceivedComposite {
  //     account_id: None,
  //     account_name: String::from("credit_union"),
  //     amount_id: None,
  //     amount_standard: 100203.34,
  //     payment_received_id: None,
  //     payment_received_completed_at: Utc::now().naive_local(),
  //     income_id: None,
  //     income_name: String::from("cowboy"),
  // };

  // payment_received_composite_test_1.create_payment_received(&mut main_store);

  // let mut payment_received_composite_test_3 = PaymentReceivedComposite {
  //     account_id: None,
  //     account_name: String::from("another_credit_union"),
  //     amount_id: None,
  //     amount_standard: 1000000000.34,
  //     payment_received_id: None,
  //     payment_received_completed_at: Utc::now().naive_local(),
  //     income_id: None,
  //     income_name: String::from("lottery"),
  // };

  // payment_received_composite_test_3.create_payment_received(&mut main_store);
