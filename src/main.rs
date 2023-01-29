use crate::error::error_handler::ErrorHandler;
use crate::programs::calendar_slice_model::CalendarSliceModel;
// temp
use crate::calendar::month::MonthKey;

mod calendar;
mod composite;
mod error;
mod programs;
mod schema;
mod storage;
mod test;
mod traits;

fn main() {
    if let Err(err) =
        CalendarSliceModel::new(2023, MonthKey::Feb, 2023, MonthKey::Mar, true, "", "").run()
    {
        ErrorHandler::log(err);
    }
}
