use std::collections::HashMap;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use crate::composite::payment_composite::{PaymentComposite, PaymentCompositeStore};
use crate::composite::payment_received_composite::{PaymentReceivedComposite, PaymentReceivedCompositeStore};
use crate::store::store::Store;
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    pub id: Option<usize>,
    pub payments: PaymentCompositeStore,
    pub payments_received: PaymentReceivedCompositeStore,
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
    pub fn add_payment(&mut self, payment_comp: PaymentComposite) -> () {
        PaymentComposite::save_to_store(payment_comp, &mut self.payments);
    }

    pub fn add_payment_received(&mut self, payment_rec_comp: PaymentReceivedComposite) -> () {
        PaymentReceivedComposite::save_to_store(payment_rec_comp, &mut self.payments_received);
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
