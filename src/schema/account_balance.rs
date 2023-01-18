use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct AccountBalance {
    pub id: usize,
    pub account_id: usize,
    pub reported_at: DateTime<Utc>,
    pub amount: f64,
}

pub type AccountBalanceStore = HashMap<usize, AccountBalance>;

impl CsvRecord<AccountBalance> for AccountBalance {
    fn id(&self) -> usize {
        self.id
    }

    fn clone_record(&self) -> AccountBalance {
        self.clone()
    }
}
impl CsvStore for AccountBalance {}
