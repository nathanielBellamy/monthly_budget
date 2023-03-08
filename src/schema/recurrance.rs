use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Recurrence {
    pub id: Option<usize>,
    pub frequency: Every,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum Every {
    Days(u64),   // Every::Days(2) = every other day
    Weeks(u64),  // Every::Weeks(2) = every other week
    Months(u32), // Every::Months(6) = twice a year
    Years(u32),  // Every::Years(10) = once per decade
}
