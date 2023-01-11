use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountBalance {
    pub id: usize,
    pub account_id: usize,
    pub date: DateTime<Utc>,
    pub amount: usize,
}
