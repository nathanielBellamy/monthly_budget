use crate::calendar::calendar_slice::CalendarSlice;
use crate::calendar::year_month::YearMonth as YM;
use crate::composite::payment_event::PaymentEvent;
use crate::programs::month_model::MonthModel;
use crate::storage::store::Store;
use std::error::Error;

pub struct CalendarSliceModel {
    start: YM,
    end: YM,
    output_results: bool,
    path_in: String,
    path_out: String,
}

type CalendarSliceModelResult = Result<(), Box<dyn Error>>;

impl CalendarSliceModel {
    pub fn new(
        start: YM,
        end: YM,
        output_results: bool,
        path_in: String,
        path_out: String,
    ) -> CalendarSliceModel {
        CalendarSliceModel {
            start,
            end,
            output_results,
            path_in,
            path_out,
        }
    }

    pub fn run(&self, dir: String) -> CalendarSliceModelResult {
        let mut store = Store::new();
        store.init(Some(self.path_in.clone()))?;

        let year_slice = CalendarSlice::new(self.start, self.end)?;
        // TODO: accept JSON or CSV
        // TODO: handle recurring events, maybe in a separate recurring_events.xxx file
        // TODO: generate payment_events from Expenses and Incomes
        //        - allow set recurrence or random
        let payment_events_path = format!("data/json/{dir}/payment_events.json");
        let payment_event_month_bins =
            PaymentEvent::fetch_and_bin_events_by_month(payment_events_path)?;
        for month in year_slice.months().iter() {
            MonthModel::new(*month, false, None, None).run(
                &payment_event_month_bins[month],
                Some(&mut store),
                None,
            )?;
        }

        if self.output_results {
            store.write_to_csv(Some(self.path_out.clone()))?;
        }

        Ok(())
    }
}
