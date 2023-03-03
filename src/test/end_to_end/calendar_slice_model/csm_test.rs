use crate::app::cli::Cli;
use crate::programs::calendar_slice_model::CalendarSliceModel;

pub struct CsmTest;

impl CsmTest {
    #[allow(unused)] // used in tests below
    pub fn run() -> u8 {
        let cli = Cli::new(
            CsmTest::format_path("structs/init/"), // input
            CsmTest::format_path("reports/"),      // output
            CsmTest::format_path("events"),        // payment_events
            "2023-02".to_string(),                 // start
            "2023-05".to_string(),                 // end
        );
        println!("CSMTEST RUN");
        match CalendarSliceModel::run_cli(cli) {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }

    pub fn format_path(path: &'static str) -> String {
        let base = "src/test/end_to_end/calendar_slice_model/data";
        format!("{}/{}", base, path)
    }
}

#[cfg(test)]
mod calendar_slice_model_e2e {
    use super::*;
    use crate::storage::store::Store;
    use std::sync::Once;

    // call test once
    // test resulting data multiple ways
    static mut RES: u8 = 1;
    static INIT: Once = Once::new();
    static STORE_INIT: &str = "src/test/end_to_end/calendar_slice_model/data/reports/";
    fn run_test() -> u8 {
        unsafe {
            INIT.call_once(|| {
                // TODO: clear previously created data
                RES = CsmTest::run();
            });
            RES
        }
    }

    #[test]
    fn check_account_balances() {
        run_test();
        let mut store = Store::new();
        store.init(Some(STORE_INIT.to_string())).unwrap();
        assert_eq!(44, store.account_balances.len());
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
