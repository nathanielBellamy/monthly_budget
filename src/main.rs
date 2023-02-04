use crate::calendar::year_month::YearMonth as YM;
use crate::error::error_handler::ErrorHandler;
use crate::programs::calendar_slice_model::CalendarSliceModel;
// temp
use crate::calendar::month_key::MonthKey as MK;

mod calendar;
mod composite;
mod error;
mod programs;
mod schema;
mod storage;
mod test;
mod traits;

fn main() {
    if let Err(err) = CalendarSliceModel::new(
        YM::new(2023, MK::Feb),
        YM::new(2023, MK::Mar),
        true,
        "data/init/",
        "data/",
    )
    .run("example_1")
    {
        ErrorHandler::log(err);
    }
}
