use crate::calendar::year_month::YearMonth as YM;
use crate::error::error_handler::ErrorHandler;
use crate::programs::calendar_slice_model::CalendarSliceModel;
// temp
use crate::app::cli::Cli;
use crate::calendar::month_key::MonthKey as MK;
use clap::Parser;

mod app;
mod calendar;
mod composite;
mod error;
mod programs;
mod schema;
mod storage;
mod test;
mod traits;

fn main() {
    let cli = Cli::parse();

    println!("Value for input: {}", cli.input);
    println!("Value for out: {}", cli.output);

    if let Err(err) = CalendarSliceModel::new(
        YM::new(2023, MK::Feb),
        YM::new(2023, MK::Mar),
        true,
        cli.input,
        cli.output,
    )
    .run("example_1".to_string())
    {
        ErrorHandler::log(err);
    }
}

//    let matches = Command::new("Monthly Budget")
//     .version("0.1")
//     .author("Nathan S <nbschieber@gmail.com>")
//     .about("Budgetary CSV & JSON Processing in Rust")
//     .arg(arg!(--input <VALUE>).required(true))
//     .arg(arg!(--output <VALUE>).required(true))
//     .get_matches();

// panic!(
//     "input: {:?}, output: {:?}",
//     matches.get_one::<String>("input").expect("required"),
//     matches.get_one::<String>("output").expect("required"),
// );
