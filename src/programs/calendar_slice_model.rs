use crate::calendar::calendar_slice::CalendarSlice;
use crate::calendar::month::MonthKey;
use crate::programs::month_model::MonthModel;
use crate::storage::store::Store;
use std::error::Error;

pub struct CalendarSliceModel {
    start_year: i32,
    start_month: MonthKey,
    end_year: i32,
    end_month: MonthKey,
    output_results: bool,
    #[allow(unused)]
    path_in: Option<&'static str>,
    #[allow(unused)]
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

    pub fn run(&self) -> CalendarSliceModelResult {
        let mut store = Store::new();
        store.init(Some("data/init/"))?;

        let year_slice = CalendarSlice::new(
            self.start_year,
            self.start_month,
            self.end_year,
            self.end_month,
        )?;

        for month in year_slice.months().iter() {
            MonthModel::new(month.0, month.1, false, None, None).run(Some(&mut store), None)?;
        }

        if self.output_results {
            store.write_to_csv(None)?;
        }

        Ok(())
    }
}
