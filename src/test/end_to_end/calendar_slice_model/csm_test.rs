use crate::app::cli::Cli;
use crate::programs::calendar_slice_model::CalendarSliceModel;

pub struct CsmTest;

impl CsmTest {
    #[allow(unused)] // used in tests below
    pub fn run() -> u8 {
        let base = "src/test/end_to_end/calendar_slice_model/data";
        let cli = Cli::new(
            CsmTest::format_path(base, "structs/init/"), // input
            CsmTest::format_path(base, "reports/"),      // output
            CsmTest::format_path(base, "events/"),       // payment_events
            "2023-02".to_string(),                       // start
            "2023-05".to_string(),                       // end
        );
        println!("CSMTEST RUN");
        match CalendarSliceModel::run_cli(cli) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }

    pub fn format_path(base: &'static str, path: &'static str) -> String {
        format!("{}/{}", base, path)
    }
}

#[cfg(test)]
mod calendar_slice_model_e2e {
    use super::*;
    use std::sync::Once;

    // call test once
    // test resulting data multiple ways
    static mut RES: u8 = 1;
    static INIT: Once = Once::new();

    fn run_test() -> u8 {
        unsafe {
            INIT.call_once(|| {
                RES = CsmTest::run();
            });
            RES
        }
    }

    #[test]
    fn check_account_balances() {
        run_test();
    }

    #[test]
    fn check_payments() {
        run_test();
        //TODO
    }

    #[test]
    fn check_payments_received() {
        run_test();
        //TODO
    }

    #[test]
    fn check_accounts() {
        run_test();
        //TODO
    }

    #[test]
    fn check_expenses() {
        run_test();
        //TODO
    }

    #[test]
    fn check_incomes() {
        run_test();
        //TODO
    }

    #[test]
    fn check_amounts() {
        run_test();
        //TODO
    }
}
