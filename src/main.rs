use clap::Parser;
use monthly_budget::app::cli::Cli;
use monthly_budget::error::error_handler::ErrorHandler;
use monthly_budget::programs::calendar_slice_model::CalendarSliceModel;
use monthly_budget::test::end_to_end::calendar_slice_model::csm_test::CsmTest;

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
