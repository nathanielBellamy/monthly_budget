use crate::app::cli::Cli;
use crate::calendar::year_month::YearMonth as YM;
use crate::error::error_handler::ErrorHandler;
use crate::programs::calendar_slice_model::CalendarSliceModel;
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

// to run:
// cargo run -- -s 2023-2 -e 2023-3
fn main() {
    let cli = Cli::parse();
    let start = YM::parse(cli.startym);
    let end = YM::parse(cli.endym);
    println!("Start from: {:?} - {:?}", start.year, start.month);
    println!("End at: {:?} - {:?}", end.year, end.month);
    println!("Inputs from: {:?}", cli.input);
    println!("Outputs to: {:?}", cli.output);
    println!("Reads PaymentEvent JSON from: {:?}", cli.transactions);

    if let Err(err) =
        CalendarSliceModel::new(start, end, true, cli.input, cli.output).run(cli.transactions)
    {
        ErrorHandler::log(err);
    }
}
