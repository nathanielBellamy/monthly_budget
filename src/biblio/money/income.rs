use crate::biblio::money::tag::Tag;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct Income {
    pub id: usize,
    pub active: bool,
    pub name: String,
    pub recurrence_id: Option<usize>,
    pub tags: Option<Vec<Tag>>,
}

impl Income {
    pub fn amount(&self) -> usize {
        //TODO: accept different parameters to make different computations
        10
    }

    pub fn read_csv_into_store(store: &mut Vec<Income>) -> Result<&'static str, Box<dyn Error>> {
        // file handle and reader
        let path = std::env::current_dir()?;
        println!("{:?}", path.display());
        let file = File::open("data/incomes.csv")?;

        let mut reader = Reader::from_reader(file);

        // Check each result, return read errors
        for result in reader.deserialize() {
            match result {
                Err(err) => return Err(From::from(err)),
                Ok(record) => {
                    let income: Income = record;
                    println!("Income: {:?}", income);
                    store.push(income);
                    println!("Income Store: {:?}", store);
                }
            }
        }
        Ok("OK")
    }
}
