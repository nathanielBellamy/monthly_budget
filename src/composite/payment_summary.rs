use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// TODO: rename Expense Summary
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PaymentSummary {
    pub id: Option<usize>,
    pub name: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub total: Decimal,
}

impl PaymentSummary {
    pub fn clone(&self) -> PaymentSummary {
        PaymentSummary {
            name: self.name.clone(),
            ..*self
        }
    }
}

pub type PaymentSummaryStore = BTreeMap<usize, PaymentSummary>;

impl CsvRecord<PaymentSummary> for PaymentSummary {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
        self.id = Some(new_id);
        self.id
    }

    fn clone_record(&self) -> PaymentSummary {
        self.clone()
    }
}

impl CsvStore<PaymentSummary> for PaymentSummary {}
