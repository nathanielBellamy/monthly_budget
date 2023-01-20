// use std::error::Error;

pub trait CsvRecord<T> {
    /*
      match record.id {
        None => record is not yet saved in main_store,
        Some(id) => record is a local copy of record in store
      }
    */
    fn id(&self) -> Option<usize>;

    fn set_id(&mut self, new_id: usize) -> Option<usize>;

    fn clone_record(&self) -> T;
}
