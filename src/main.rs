use crate::error::error_handler::ErrorHandler;
// temp
use crate::calendar::month::MonthKey;
use crate::programs::month_model::MonthModel;

mod calendar;
mod composite;
mod error;
mod programs;
mod schema;
mod storage;
mod test;
mod traits;

fn main() {
    if let Err(err) = MonthModel::new(MonthKey::Feb, None, None).run() {
        ErrorHandler::log(err);
    }
}
