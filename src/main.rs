use chrono::{Utc};
use crate::composite::payment_composite::PaymentComposite;
use crate::composite::payment_received_composite::PaymentReceivedComposite;
use crate::error_handler::error_handler::ErrorHandler;
// use crate::error_handler::error_handler::ErrorHandler;
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
  let mut main_store: Store = Store::new(); // main_store is data owned by main()

  // if let Err(_) = main_store.init(None) {
  //     ErrorHandler::log(From::from(format!("An error occured when initilizing main_store")));
  // }


  // main_store.init(None);

  // main_store.write_to_csv(None);


  MonthModel::new(MonthKey::Feb).run();

}
