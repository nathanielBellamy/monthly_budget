pub mod _csv {
    use std::error::Error;
    use std::fs::File;

    pub fn read_csv_into_store(file_path: &str) -> Result<&str, Box<dyn Error>> {
        // file handle and reader
        let file = File::open(file_path)?;
        let mut reader = csv::Reader::from_reader(file);

        // Check each result, return read errors
        for result in reader.records() {
            match result {
                Err(err) => return Err(From::from(err)),
                Ok(record) => {
                    println!("HERE: {:?}", record);
                    drop(record);
                    ()
                }
            }
        }
        return Ok("OK");
    }
}
