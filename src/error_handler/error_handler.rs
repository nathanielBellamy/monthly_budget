use chrono;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct ErrorHandler;

impl ErrorHandler {
    pub fn log(err: Box<dyn Error>) -> () {
        println!("{:?}", err);
        write_error_to_log(err);
    }
}

fn write_error_to_log(err: Box<dyn Error>) -> () {
    let error: String = format!(
        "{:?} - {:?}",
        chrono::offset::Local::now().to_string(),
        err.to_string()
    );

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("src/error_handler/error_log");

    match file {
        Err(e) => println!("ERROR LOG FILE LOAD ERROR: {:?}", e),
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", error) {
                println!("ERROR LOG ERROR: {:?}", e);
            }
        }
    }
}
