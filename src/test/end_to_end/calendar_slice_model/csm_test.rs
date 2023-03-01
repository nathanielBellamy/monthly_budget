use crate::app::cli::Cli;
use crate::programs::calendar_slice_model::{CalendarSliceModel, CalendarSliceModelResult};

pub struct CsmTest;

impl CsmTest {
    pub fn run() -> CalendarSliceModelResult {
        let cli = Cli::new(
            "data/structs".to_string(), // input
            "data/reports".to_string(), // output
            "data/events".to_string(), // payment_events
            "2023-02".to_string(), // start
            "2023-05".to_string() // end
        );
        CalendarSliceModel::run_cli(&cli)
    }
}

#[cfg(test)]
mod calendar_slice_model_e2e {
    use super::*;
    use std::sync::Once;

    // call test once
    // test resulting data multiple ways
    static mut RES: CalendarSliceModelResult = Ok(());
    static INIT: Once = Once::new();   

    fn run_test() -> CalendarSliceModelResult {
        unsafe {
            INIT.call_once(|| {
                RES = CsmTest::run();
            });
            RES
        }
    }

    #[test]
    fn check_account_balances() {
        if let Err(e) = run_test(){ 
            panic!("Mistakes were made.") 
        }
        //TODO
    }

    #[test]
    fn check_payments() {
        if let Err(e) = run_test(){ 
            panic!("Mistakes were made.") 
        }       
        //TODO
    }

    #[test]
    fn check_payments_received() {
        if let Err(e) = run_test(){ 
            panic!("Mistakes were made.") 
        }  
        //TODO
    }

    #[test]
    fn check_accounts() {
         if let Err(e) = run_test(){ 
            panic!("Mistakes were made.") 
        }
        //TODO
    }

    #[test]
    fn check_expenses() {
        if let Err(e) = run_test(){ 
            panic!("Mistakes were made.") 
        }
        //TODO
    }

    #[test]
    fn check_incomes() {
        if let Err(e) = run_test(){ 
            panic!("Mistakes were made.") 
        }
        //TODO
    }

    #[test]
    fn check_amounts() {
        if let Err(e) = run_test(){ 
            panic!("Mistakes were made.") 
        }
        //TODO
    }
}
