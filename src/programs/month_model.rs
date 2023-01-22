use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use chrono::{NaiveDate};
use std::error::Error;
use crate::calendar::month::{MonthKey, Month};
use crate::calendar::day::{Day, DayStore};
use crate::composite::payment_composite::{PaymentComposite, PaymentCompositeStore};
use crate::composite::payment_received_composite::{PaymentReceivedComposite, PaymentReceivedCompositeStore};
use crate::store::store::Store;


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
    let mut store = Store::new();
    store.init(None)?;


    let mut month = Month {
      key: self.key,
      days: MonthModel::construct_days(self.key),
    };

    for (id, day) in &mut month.days {
      let id_f = *id as f64;
      let id_32 = *id as u32;
      day.add_payment(PaymentComposite {
        id: None,
        account_id: None,
        account_name: String::from("piggybank"),
        amount_id: None,
        amount_standard: id_f * 1000.00,
        payment_id: None,
        payment_completed_at: NaiveDate::from_ymd_opt(2023, Month::id(self.key), id_32).unwrap()
                                        .and_hms_opt(1, 1, 1).unwrap(),
        expense_id: None,
        expense_name: format!("Payment {:?}", id),
      });
      day.add_payment(PaymentComposite {
        id: None,
        account_id: None,
        account_name: String::from("piggybank"),
        amount_id: None,
        amount_standard: id_f * 2000.00,
        payment_id: None,
        payment_completed_at: NaiveDate::from_ymd_opt(2023, Month::id(self.key), id_32).unwrap()
                                        .and_hms_opt(3, 3, 3).unwrap(),
        expense_id: None,
        expense_name: format!("Payment 2 - {:?}", id),
      });
      day.add_payment_received(PaymentReceivedComposite {
        id: None,
        account_id: None,
        account_name: String::from("swearjar"),
        amount_id: None,
        amount_standard: 10000.34,
        payment_received_id: None,
        payment_received_completed_at: NaiveDate::from_ymd_opt(2023, Month::id(self.key), id_32).unwrap()
                                                  .and_hms_opt(2, 2, 2).unwrap(),
        income_id: None,
        income_name: String::from("lottery"),
      });
    }

    for (_id, day) in month.days.iter_mut() { // iter sorted by key thx to btree_map
        day.execute_payments_in_order(&mut store)?;
    }

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

  // payment_received_composite_test_3.create_payment_received(&mut main_store);
