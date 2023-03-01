use crate::app::cli::Cli;
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

fn main() {
    let cli = Cli::parse();
    if let Err(err) = CalendarSliceModel::run_cli(&cli) {
        ErrorHandler::log(err);
    }
}
