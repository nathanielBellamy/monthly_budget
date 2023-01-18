// use std::error::Error;

pub trait CsvRecord<T> {
    fn id(&self) -> usize;

    fn clone_record(&self) -> T;
}
