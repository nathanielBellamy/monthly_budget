use crate::app::cli::Cli;
use crate::error::error_handler::ErrorHandler;
use crate::programs::calendar_slice_model::CalendarSliceModel;
use clap::Parser;
use test::end_to_end::calendar_slice_model::csm_test::CsmTest;

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
    if cli.x_test == "t" {
        println!("Running test...");
        if CsmTest::run(cli.start_yyyy_mm, cli.end_yyyy_mm) == 1 {
            panic!("Oh NO!");
        };
        println!("Test Complete")
    } else {
        // main
        if let Err(err) = CalendarSliceModel::run_cli(cli) {
            ErrorHandler::log(err);
        }
    }
}
