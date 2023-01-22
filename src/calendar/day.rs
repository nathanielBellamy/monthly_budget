use std::error::Error;
use chrono::NaiveDateTime;
use std::collections::BTreeMap;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use crate::composite::payment_composite::{PaymentComposite, PaymentCompositeStore};
use crate::composite::payment_received_composite::{PaymentReceivedComposite, PaymentReceivedCompositeStore};
use crate::store::store::Store;
use chrono::{NaiveDate};
use std::collections::btree_map::Entry;
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

pub type DayStore = BTreeMap<usize, Day>;

impl Day {
    pub fn add_payment(&mut self, payment_comp: PaymentComposite) -> () {
        PaymentComposite::save_to_store(payment_comp, &mut self.payments);
    }

    pub fn add_payment_received(&mut self, payment_rec_comp: PaymentReceivedComposite) -> () {
        PaymentReceivedComposite::save_to_store(payment_rec_comp, &mut self.payments_received);
    }

    pub fn execute_payments_in_order(&mut self, store: &mut Store) -> Result<(), Box<dyn Error>> {
      let mut payment_times: Vec<(usize, NaiveDateTime, &str)> = vec![];
      for (id, pymnt) in self.payments.iter() {
        payment_times.push((*id, pymnt.payment_completed_at, "payment"));
      }

      for (id, pymnt_rec) in self.payments_received.iter() {
        payment_times.push((*id, pymnt_rec.payment_received_completed_at, "payment_received"))
      }

      payment_times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

      for pymnt in payment_times.iter(){
        match pymnt.2 {
          "payment" => {
            if let Entry::Occupied(mut record) = self.payments.entry(pymnt.0) {
              record.get_mut().create_payment(store, Some(pymnt.1))?;
            }
          },
          "payment_received" => {
            if let Entry::Occupied(mut record) = self.payments_received.entry(pymnt.0) {
              record.get_mut().create_payment_received(store, Some(pymnt.1))?;
            }
          },
          _ => ()
        }
      }
      Ok(())
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
