use chrono::NaiveDateTime;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PaymentDisplay {
  pub id: Option<usize>,
  pub name: String,
  pub amount: f64,
  pub account_name: String,
  pub completed_at: NaiveDateTime,
  pub prev_balance: Option<f64>,
  pub ending_balance: Option<f64>,
}

impl CsvRecord<PaymentDisplay> for PaymentDisplay {
  fn id(&self) -> Option<usize> {
        self.id
    }

  fn set_id(&mut self, new_id: usize) -> Option<usize> {
    self.id = Some(new_id);
    self.id
  }

  fn clone_record(&self) -> PaymentDisplay {
      PaymentDisplay {
        name: self.name.clone(),
        account_name: self.account_name.clone(),
        ..*self
      }
  }
}

impl CsvStore<PaymentDisplay> for PaymentDisplay {}

pub type PaymentDisplayStore = BTreeMap<usize, PaymentDisplay>;
