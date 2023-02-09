use crate::calendar::calendar_slice::CalendarSlice;
use crate::calendar::year_month::YearMonth as YM;
use crate::composite::payment_event::PaymentEventBinStore;
use crate::composite::payment_event::PaymentEventStore;
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

    pub fn run(&self) -> CalendarSliceModelResult {
        let mut store = Store::new();
        store.init(Some(self.path_in.clone()))?;

        let year_slice = CalendarSlice::new(self.start, self.end)?;
        let mut payment_event_month_bins = PaymentEventBinStore::new();
        for month in year_slice.months().iter() {
            let bin_store = payment_event_month_bins
                .entry(*month)
                .or_insert(PaymentEventStore::new());
            MonthModel::new(*month, false, None, None).run(&bin_store, Some(&mut store), None)?;
        }

        if self.output_results {
            store.write_to_csv(Some(self.path_out.clone()))?;
        }

        println!("Payment Event Bins: {:?}", payment_event_month_bins);

        Ok(())
    }
}
