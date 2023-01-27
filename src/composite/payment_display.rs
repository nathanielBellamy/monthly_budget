use chrono::NaiveDateTime;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use std::collections::BTreeMap;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PaymentDisplay {
  pub id: Option<usize>,
  pub name: String,
  #[serde(with = "rust_decimal::serde::float")]
  pub amount: Decimal,
  pub account_name: String,
  pub completed_at: NaiveDateTime,
  #[serde(with = "rust_decimal::serde::float_option")]
  pub prev_balance: Option<Decimal>,
  #[serde(with = "rust_decimal::serde::float_option")]
  pub ending_balance: Option<Decimal>,
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
