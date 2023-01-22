use crate::traits::csv_record::CsvRecord;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use crate::error_handler::error_handler::ErrorHandler;


pub type CsvReadResult = Result<(), Box<dyn Error>>;
pub type CsvWriteResult = Result<(), Box<dyn Error>>;

pub trait CsvStore<T: for<'a> Deserialize<'a> + for<'a> Serialize + std::fmt::Debug + CsvRecord<T> + CsvStore<T>> {
    fn init_store(
        store: &mut BTreeMap<usize, T>,
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

    fn write_to_csv(
        store: & BTreeMap<usize, T>,
        path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_path(path)?;

        for (_id, record) in store.iter() {
          wtr.serialize(record.clone_record())?;
        }

        wtr.flush()?;
        Ok(())
    }

    fn new_id(csv_store: &BTreeMap<usize, T>) -> usize {
        let mut max_id: usize = 0;
        for (id, _record) in csv_store {
            if *id > max_id {
                max_id = *id
            }
        }
        max_id + 1
    }

    fn save_to_store(mut record: T, csv_store: &mut BTreeMap<usize, T>) -> usize { // returns id newly saved record
        #[allow(unused_assignments)]
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

    fn by_id(id: usize, csv_store: &mut BTreeMap<usize, T>) -> Option<T> {
        match csv_store.entry(id) {
            Entry::Vacant(_) => None,
            Entry::Occupied(record) => {
                let rec: T = record.get().clone_record();
                Some(rec)
            }
        }
    }
}
