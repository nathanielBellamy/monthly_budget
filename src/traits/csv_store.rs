use crate::traits::csv_record::CsvRecord;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub type CsvReadResult = Result<(), Box<dyn Error>>;

pub trait CsvStore {
    fn init_store<T: for<'a> Deserialize<'a> + std::fmt::Debug + CsvRecord<T>>(
        store: &mut HashMap<usize, T>,
        csv_path: &str,
    ) -> CsvReadResult {
        let file = File::open(csv_path)?;
        let mut reader = Reader::from_reader(file);

        for result in reader.deserialize() {
            match result {
                Err(err) => return Err(From::from(err)),
                Ok(res) => {
                    let record: T = res; // type hint for .deserialize
                    let id: usize = record.id();
                    store.entry(id).or_insert(record);
                }
            }
        }
        Ok(())
    }

    // TODO
    fn _write_csv_store<T: Serialize + std::fmt::Debug>(
        _store: &mut Vec<T>,
        _csv_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn new_id<T>(csv_store: &HashMap<usize, T>) -> usize {
        let mut max_id: usize = 0;
        for (id, _record) in csv_store {
            if *id > max_id {
                max_id = *id
            }
        }
        max_id + 1
    }

    fn save_to_store<T: CsvRecord<T>>(record: T, csv_store: &mut HashMap<usize, T>) -> &mut T {
        csv_store.entry(record.id()).or_insert(record)
    }

    fn by_id<T: CsvRecord<T>>(id: usize, csv_store: &mut HashMap<usize, T>) -> Option<T> {
        match csv_store.entry(id) {
            Entry::Vacant(_) => None,
            Entry::Occupied(record) => {
                let rec: T = record.get().clone_record();
                Some(rec)
            }
        }
    }
}
