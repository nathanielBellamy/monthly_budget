use crate::time::Date;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Recurrence {
    pub id: usize,
    pub start_date: Date,
    pub end_date: Date,
    pub cycle_length: CycleLength,
}

pub enum CycleLength {
    Day(u8),
    Week(u8),
    Month(u8),
    Year(u8),
}
