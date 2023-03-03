use crate::app::cli::Cli;
use crate::programs::calendar_slice_model::CalendarSliceModel;

pub struct CsmTest;

impl CsmTest {
    #[allow(unused)] // used in tests below
    pub fn run() -> u8 {
        let cli = Cli::new(
            CsmTest::format_path("init/"), // input
            CsmTest::format_path("reports/"),      // output
            CsmTest::format_path("events"),        // payment_events
            "2023-03".to_string(),                 // start
            "2023-03".to_string(),                 // end
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
    use std::fs;
    use crate::storage::store::Store;
    use rust_decimal::Decimal;
    use std::sync::Once;

    // call test once
    // test resulting data multiple ways
    static mut RES: u8 = 1;
    static INIT: Once = Once::new();
    static STORE_INIT: &str = "src/test/end_to_end/calendar_slice_model/data/reports/";
    fn run_test() -> u8 {
        unsafe {
            INIT.call_once(|| {
                clean_up_previous_data();
                RES = CsmTest::run();
            });
            RES
        }
    }

    fn clean_up_previous_data() {
        fs::remove_dir_all(STORE_INIT).unwrap();
        fs::create_dir(STORE_INIT).unwrap();
    }

    #[test]
    fn check_account_balances() {
        run_test();
        let mut store = Store::new();
        store.init(Some(STORE_INIT.to_string())).unwrap();
        assert_eq!(12, store.account_balances.len());
        let final_balance = store.account_balances[&12];
        assert_eq!(2, final_balance.account_id);
        assert_eq!(Decimal::new(8500,0), final_balance.amount);
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
