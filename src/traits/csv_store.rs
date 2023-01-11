pub trait Store {
  //TODO: impl hash store
  fn new(&self, store: Vec<&impl RecordStruct>){
    store
  }

  fn read_csv_from<'a, T>(&self, path: &'a str, store: &'a Vec<T>) -> Result<(), csv::Error> {
    let csv = fs::read_to_string(path);

    if let Ok(csv_string) = csv {
      let mut reader = csv::Reader::from_reader(csv_string.as_bytes());

      for record in reader.deserialize() {
          let record: T = record?;
          store.push(record)
      }
    }

    Ok(())
  }
}
