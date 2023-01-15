use crate::error_handler::error_handler::ErrorHandler;
use crate::schema::calendar;
use crate::store::store::Store;

mod error_handler;
mod schema;
mod spec;
mod store;
mod traits;

fn main() {
    let mut main_store: Store = Store::new(); // main_store is data owned by main()
    let mut store: Option<&mut Store> = None; // store is mutable reference to this data to be passed
    match main_store.init(None) {
        Err(err) => ErrorHandler::log(err),
        Ok(res) => store = Some(res),
    }

    match store {
        None => println!("An error occured when initilizing Store."),
        Some(data) => println!("You have initiated Store: \n {:?}", data),
    }
}
