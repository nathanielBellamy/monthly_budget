use crate::traits::csv_record::CsvRecord;
use csv::Reader;
use serde::Deserialize;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use crate::error_handler::error_handler::ErrorHandler;


pub type CsvReadResult = Result<(), Box<dyn Error>>;

pub trait CsvStore<T: for<'a> Deserialize<'a> + std::fmt::Debug + CsvRecord<T> + CsvStore<T>> {
    fn init_store(
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
                    let id: usize = record.id().unwrap();
                    store.entry(id).or_insert(record);
                }
            }
        }
        Ok(())
    }

    // TODO
    fn _write_csv_store(
        _store: &mut Vec<T>,
        _csv_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn new_id(csv_store: &HashMap<usize, T>) -> usize {
        let mut max_id: usize = 0;
        for (id, _record) in csv_store {
            if *id > max_id {
                max_id = *id
            }
        }
        max_id + 1
    }

    fn save_to_store(mut record: T, csv_store: &mut HashMap<usize, T>) -> usize {
        let mut new_id: usize = 0;
        match record.id() {
          None => {
            new_id = T::new_id(csv_store);
            record.set_id(new_id);
            csv_store.entry(new_id).or_insert(record);
          },
          Some(id) => {
            new_id = id;
            csv_store.entry(id).or_insert(record);
          }
        }

        if let Entry::Vacant(_) = csv_store.entry(new_id) {
          ErrorHandler::log(From::from(format!("Error saving new {:?} record: {:?}", std::any::type_name::<T>(), new_id)));
          return 0;
        }

        new_id
    }

    fn by_id(id: usize, csv_store: &mut HashMap<usize, T>) -> Option<T> {
        match csv_store.entry(id) {
            Entry::Vacant(_) => None,
            Entry::Occupied(record) => {
                let rec: T = record.get().clone_record();
                Some(rec)
            }
        }
    }
}
