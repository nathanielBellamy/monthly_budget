use crate::app::cli::Cli;
use crate::programs::calendar_slice_model::CalendarSliceModel;

pub struct CsmTest;

impl CsmTest {
    pub fn run() {
        let cli = Cli::new(); //TODO
        if let Err(e) = CalendarSliceModel::run_cli(&cli) {
            panic!("Mistakes were made.")
        }
    }
}
