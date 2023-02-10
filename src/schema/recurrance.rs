use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Recurrence {
    pub id: Option<usize>,
    pub frequency: Every,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Every {
    Days(u64),   // Every::Days(2) = every other day
    Weeks(u64),  // Every::Weeks(2) = every other week
    Months(u32), // Every::Months(6) = twice a year
    Years(u32),  // Every::Years(10) = once per decade
}

#[allow(unused)]
pub type RecurrenceStore = BTreeMap<usize, Recurrence>;

impl CsvRecord<Recurrence> for Recurrence {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
        self.id = Some(new_id);
        self.id
    }

    fn clone_record(&self) -> Recurrence {
        unimplemented!()
    }
}
impl CsvStore<Recurrence> for Recurrence {}
