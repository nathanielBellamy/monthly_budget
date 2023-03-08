use crate::app::cli::Cli;
use crate::calendar::calendar_slice::CalendarSlice;
use crate::calendar::year_month::YearMonth as YM;
use crate::composite::account_summary::AccountSummary;
use crate::composite::payment_event::PaymentEvent;
use crate::composite::payment_event::PaymentEventBinStore;
use crate::composite::payment_summary::{PaymentSummary, PaymentSummaryStore};
use crate::composite::recurring_payment_event::RecurringPaymentEvent;
use crate::programs::month_model::MonthModel;
use crate::schema::expense::Expense;
use crate::schema::income::Income;
use crate::storage::store::Store;
use crate::traits::csv_store::CsvStore;
use crate::traits::file_io::FileIO;
use std::error::Error;

pub struct CalendarSliceModel {
    start: YM,
    end: YM,
    output_results: bool,
    path: String,
}

pub type CalendarSliceModelResult = Result<(), Box<dyn Error>>;

impl FileIO<CalendarSliceModel> for CalendarSliceModel {
    fn path(&self) -> String {
        self.path.clone()
    }
}

impl CalendarSliceModel {
    pub fn new(start: YM, end: YM, path: String, output_results: bool) -> CalendarSliceModel {
        CalendarSliceModel {
            start,
            end,
            path,
            output_results,
        }
    }

    pub fn run_cli(cli: Cli) -> CalendarSliceModelResult {
        let start = YM::parse(cli.start_yyyy_mm);
        let end = YM::parse(cli.end_yyyy_mm);
        println!("Running from Cli...");
        println!("Using data from {:?}", cli.path);
        println!("Start from: {:?} - {:?}", start.year, start.month);
        println!("End at: {:?} - {:?}", end.year, end.month);

        CalendarSliceModel::new(start, end, cli.path, true).run()
    }

    pub fn run(&self) -> CalendarSliceModelResult {
        println!(
            "Running Calendar Slice Model From: {:#?}-{:#?} to {:#?}-{:#?}",
            self.start.year, self.start.month, self.end.year, self.end.month
        );

        let mut store = Store::new();
        store.init(Some(self.path_in()))?;

        let cal_slice = CalendarSlice::new(self.start, self.end)?;
        let mut payment_event_month_bins = PaymentEventBinStore::new();

        let recurring_events_path = format!("{}/{}", self.path_events(), "reccurring.json");
        RecurringPaymentEvent::fetch_and_bin_recurring_events(
            recurring_events_path,
            &cal_slice,
            &mut payment_event_month_bins,
        )?;

        let one_off_events_path = format!("{}/{}", self.path_events(), "one_off.json");
        PaymentEvent::fetch_and_bin_one_off_events(
            one_off_events_path,
            &cal_slice,
            &mut payment_event_month_bins,
        )?;

        for month in cal_slice.months().iter() {
            // year_months in chrono order thx to Eq, PartialEq, PartialOrd, Ord Traits and BTreeMap
            let pe_bin_store = payment_event_month_bins.entry(*month).or_default();
            MonthModel::new(*month, self.path(), true).run(pe_bin_store, Some(&mut store))?;
        }

        if self.output_results {
            let account_ids: Vec<usize> = store.accounts.keys().cloned().collect();
            for id in account_ids.iter() {
                let account_summary_store = AccountSummary::by_id(*id, &mut store);
                let path = self.format_path(format!("account_{}_summary", *id));
                AccountSummary::write_to_csv(&account_summary_store, path.as_str())?;
            }

            let expense_summary = CalendarSliceModel::construct_payment_summary(&mut store);
            PaymentSummary::write_to_csv(
                &expense_summary,
                self.format_path("expense_summary".to_string()).as_str(),
            )?;

            let income_summary = CalendarSliceModel::construct_payment_received_summary(&mut store);
            PaymentSummary::write_to_csv(
                &income_summary,
                self.format_path("income_summary".to_string()).as_str(),
            )?;

            // write main store
            store.write_to_csv(Some(self.path_out()))?;
        }

        // println!("Payment Event Bins: {payment_event_month_bins:#?}");
        println!("===============================================");
        // println!("Final Store: {store:#?}");

        Ok(())
    }

    pub fn format_path(&self, path: String) -> String {
        format!("{}/{}.csv", self.path_out(), path)
    }

    pub fn construct_payment_summary(store: &mut Store) -> PaymentSummaryStore {
        let mut payment_summary_store = PaymentSummaryStore::new();
        let expense_ids: Vec<usize> = store.expenses.keys().cloned().collect();
        for expense_id in expense_ids {
            // sorted expense ids
            payment_summary_store
                .entry(expense_id)
                .or_insert(PaymentSummary {
                    id: Some(expense_id),
                    name: Expense::name_by_id(expense_id, &mut store.expenses).to_string(),
                    total: Expense::total_by_id(expense_id, store),
                });
        }
        payment_summary_store
    }

    pub fn construct_payment_received_summary(store: &mut Store) -> PaymentSummaryStore {
        let mut payment_received_summary_store = PaymentSummaryStore::new();
        let income_ids: Vec<usize> = store.incomes.keys().cloned().collect();
        for income_id in income_ids {
            // sorted expense ids
            payment_received_summary_store
                .entry(income_id)
                .or_insert(PaymentSummary {
                    id: Some(income_id),
                    name: Income::name_by_id(income_id, &mut store.incomes).to_string(),
                    total: Income::total_by_id(income_id, store),
                });
        }
        payment_received_summary_store
    }
}
