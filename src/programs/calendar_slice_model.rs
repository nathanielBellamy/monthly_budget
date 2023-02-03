use crate::calendar::calendar_slice::CalendarSlice;
use crate::calendar::month::MonthKey;
use crate::calendar::year_month::YearMonth;
use crate::composite::payment_event::PaymentEvent;
use crate::programs::month_model::MonthModel;
use crate::storage::store::Store;
use std::error::Error;

pub struct CalendarSliceModel {
    start_year: i32,
    start_month: MonthKey,
    end_year: i32,
    end_month: MonthKey,
    output_results: bool,
    path_in: Option<&'static str>,
    path_out: Option<&'static str>,
}

type CalendarSliceModelResult = Result<(), Box<dyn Error>>;

impl CalendarSliceModel {
    pub fn new(
        start_year: i32,
        start_month: MonthKey,
        end_year: i32,
        end_month: MonthKey,
        output_results: bool,
        path_in: &'static str,
        path_out: &'static str,
    ) -> CalendarSliceModel {
        CalendarSliceModel {
            start_year,
            start_month,
            end_year,
            end_month,
            output_results,
            path_in: Some(path_in),
            path_out: Some(path_out),
        }
    }

    pub fn run(&self, dir: &'static str) -> CalendarSliceModelResult {
        let mut store = Store::new();
        store.init(self.path_in)?;

        let year_slice = CalendarSlice::new(
            self.start_year,
            self.start_month,
            self.end_year,
            self.end_month,
        )?;
        // TODO: accept JSON or CSV
        // TODO: handle recurring events, maybe in a separate recurring_events.xxx file
        // TODO: generate payment_events from Expenses and Incomes
        //        - allow set recurrence or random
        let payment_events_path = format!("data/json/{dir}/payment_events.json");
        let payment_event_month_bins =
            PaymentEvent::fetch_and_bin_events_by_month(payment_events_path)?;
        for month in year_slice.months().iter() {
            MonthModel::new(YearMonth(month.0, month.1), false, None, None).run(
                &payment_event_month_bins[&YearMonth(month.0, month.1)],
                Some(&mut store),
                None,
            )?;
        }

        if self.output_results {
            store.write_to_csv(self.path_out)?;
        }

        Ok(())
    }
}
