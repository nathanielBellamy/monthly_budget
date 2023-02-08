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

    println!("Inputs from: {:?}", cli.input);
    println!("Outputs to: {:?}", cli.output);
    println!("Reads PaymentEvent JSON from: {:?}", cli.example);

    if let Err(err) = CalendarSliceModel::new(
        YM::new(2023, MK::Feb),
        YM::new(2023, MK::Mar),
        true,
        cli.input,
        cli.output,
    )
    .run(cli.example)
    {
        ErrorHandler::log(err);
    }
}
