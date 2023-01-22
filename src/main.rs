use crate::error_handler::error_handler::ErrorHandler;
use crate::store::store::Store;
//temp
use crate::programs::month_model::MonthModel;
use crate::calendar::month::MonthKey;

mod calendar;
mod composite;
mod error_handler;
mod programs;
mod schema;
mod spec;
mod store;
mod traits;

fn main() {
  if let Err(err) = MonthModel::new(MonthKey::Feb).run() {
    ErrorHandler::log(From::from(err));
  }
}
