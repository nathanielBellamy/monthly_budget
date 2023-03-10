use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct AccountBalance {
    pub id: Option<usize>,
    pub account_id: usize,
    pub reported_at: NaiveDateTime,
    #[serde(with = "rust_decimal::serde::float")]
    pub amount: Decimal,
}

pub type AccountBalanceStore = BTreeMap<usize, AccountBalance>;

impl CsvRecord<AccountBalance> for AccountBalance {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
        self.id = Some(new_id);
        self.id
    }

    fn clone_record(&self) -> AccountBalance {
        *self
    }
}
impl CsvStore<AccountBalance> for AccountBalance {}
