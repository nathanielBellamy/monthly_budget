use csv::Reader;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

pub trait CsvStore {
    //TODO: init_store_hash, init_store_my_data_structure
    fn init_store_vec<T: for<'a> Deserialize<'a> + std::fmt::Debug>(
        store: &mut Vec<T>,
        csv_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let file = File::open(csv_path)?;
        let mut reader = Reader::from_reader(file);

        for result in reader.deserialize() {
            match result {
                Err(err) => return Err(From::from(err)),
                Ok(res) => {
                    let record: T = res;
                    store.push(record);
                }
            }
        }
        println!("Store: {:?}", store);
        Ok(())
    }

    // TODO
    fn write_csv_store_vec<T: Serialize + std::fmt::Debug>(
        _store: &mut Vec<T>,
        _csv_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
