use std::collections::HashMap;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use crate::schema::payment::Payment;
use crate::composite::payment_composite::PaymentComposite;
use crate::composite::payment_received_composite::PaymentReceivedComposite;
use crate::store::store::Store;
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// TODO: payments: PaymentCompositeStore, payments_received: PaymentReceivedCompositeStore
// Turn it all into Csv Stuff
#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    pub id: Option<usize>,
    pub payments: Vec<PaymentComposite>,
    pub payments_received: Vec<PaymentReceivedComposite>,
    pub date: NaiveDate,
}


impl CsvRecord<Day> for Day {
  fn id(&self) -> Option<usize> {
        self.id
    }

  fn set_id(&mut self, new_id: usize) -> Option<usize> {
      self.id = Some(new_id);
      self.id
    }

  fn clone_record(&self) -> Day {
      unimplemented!();
  }
}

impl CsvStore<Day> for Day{}

pub type DayStore = HashMap<usize, Day>;

impl Day {
    pub fn add_payment(&mut self, payment: PaymentComposite) -> () {
        self.payments.push(payment)
    }

    pub fn add_payment_received(&mut self, payment_received: PaymentReceivedComposite) -> () {
        self.payments_received.push(payment_received)
    }
}

#[cfg(test)]
mod day_spec {
    // use super::*;
    // use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn add_payment__adds_payment_to_self_payments() {
      assert_eq!(2, 2);
    }
}
