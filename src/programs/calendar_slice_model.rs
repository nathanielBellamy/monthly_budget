use crate::app::cli::Cli;
use crate::calendar::calendar_slice::CalendarSlice;
use crate::calendar::year_month::YearMonth as YM;
use crate::composite::payment_event::PaymentEvent;
use crate::composite::payment_event::PaymentEventBinStore;
use crate::composite::recurring_payment_event::RecurringPaymentEvent;
use crate::programs::month_model::MonthModel;
use crate::storage::store::Store;
use std::error::Error;

pub struct CalendarSliceModel {
    start: YM,
    end: YM,
    output_results: bool,
    path_in: String,
    path_out: String,
    events_path: String,
}

pub type CalendarSliceModelResult = Result<(), Box<dyn Error>>;

impl CalendarSliceModel {
    pub fn new(
        start: YM,
        end: YM,
        output_results: bool,
        path_in: String,
        path_out: String,
        events_path: String,
    ) -> CalendarSliceModel {
        CalendarSliceModel {
            start,
            end,
            output_results,
            path_in,
            path_out,
            events_path,
        }
    }

    pub fn run_cli(cli: Cli) -> CalendarSliceModelResult {
        let start = YM::parse(cli.start_yyyy_mm);
        let end = YM::parse(cli.end_yyyy_mm);
        println!("Running from Cli...");
        println!("Start from: {:?} - {:?}", start.year, start.month);
        println!("End at: {:?} - {:?}", end.year, end.month);
        println!("Inputs from: {:?}", cli.input);
        println!("Outputs to: {:?}", cli.output);

        CalendarSliceModel::new(start, end, true, cli.input, cli.output, cli.payment_events).run()
    }

    pub fn run(&self) -> CalendarSliceModelResult {
        println!(
            "Running Calendar Slice Model From: {:#?}-{:#?} to {:#?}-{:#?}",
            self.start.year, self.start.month, self.end.year, self.end.month
        );

        let mut store = Store::new();
        store.init(Some(self.path_in.clone()))?;

        let cal_slice = CalendarSlice::new(self.start, self.end)?;
        let mut payment_event_month_bins = PaymentEventBinStore::new();

        let recurring_events_path = format!("{}/{}", self.events_path, "reccurring.json");
        RecurringPaymentEvent::fetch_and_bin_recurring_events(
            recurring_events_path,
            &cal_slice,
            &mut payment_event_month_bins,
        )?;

        let one_off_events_path = format!("{}/{}", self.events_path, "one_off.json");
        PaymentEvent::fetch_and_bin_one_off_events(
            one_off_events_path,
            &cal_slice,
            &mut payment_event_month_bins,
        )?;

        for month in cal_slice.months().iter() {
            // year_months in chrono order thx to Eq, PartialEq, PartialOrd, Ord Traits and BTreeMap
            let pe_bin_store = payment_event_month_bins.entry(*month).or_default();
            MonthModel::new(
                *month,
                true,
                Some(self.path_in.clone()),
                Some(self.path_out.clone()),
            )
            .run(pe_bin_store, Some(&mut store))?;
        }

        if self.output_results {
            store.write_to_csv(Some(self.path_out.clone()))?;
        }

        println!("Payment Event Bins: {payment_event_month_bins:#?}");
        println!("===============================================");
        println!("Final Store: {store:#?}");

        Ok(())
    }
}
