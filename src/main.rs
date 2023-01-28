use crate::error::error_handler::ErrorHandler;
// temp
use crate::programs::month_model::MonthModel;
use crate::calendar::month::MonthKey;

mod calendar;
mod composite;
mod error;
mod programs;
mod schema;
mod test;
mod storage;
mod traits;

fn main() {
  if let Err(err) = MonthModel::new(MonthKey::Feb, None, None).run() {
    ErrorHandler::log(err);
  }
}
