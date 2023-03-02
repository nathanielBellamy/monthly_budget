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
//    let cli = Cli::parse();
//    if let Err(err) = CalendarSliceModel::run_cli(cli) {
//        ErrorHandler::log(err);
//    }
    if CsmTest::run() == 1 {
        panic!("Oh NO!");
    };
}
