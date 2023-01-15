use crate::traits::csv_store::CsvStore;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct AccountBalance {
    pub id: usize,
    pub account_id: usize,
    pub reported_at: DateTime<Utc>,
    pub amount: f64,
}

impl CsvStore for AccountBalance {}
