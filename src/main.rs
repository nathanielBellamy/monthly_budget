use chrono::Utc;
// use crate::error_handler::error_handler::ErrorHandler;
use crate::store::store::Store;
//temp
use crate::composite::payment_composite::PaymentComposite;

mod calendar;
mod composite;
mod error_handler;
mod schema;
mod spec;
mod store;
mod traits;

fn main() {
  let mut main_store: Store = Store::new(); // main_store is data owned by main()

  match main_store.init(None) {
      Err(err) => println!("An error occured when initilizing main_store: {:?}", err),
      Ok(data) => println!("You have initiated main_store: \n {:?}", data),
  }


  let mut payment_composite_test_1 = PaymentComposite {
    account_id: None,
    account_name: String::from("piggybank"),
    amount_id: None,
    amount_standard: 1234.56,
    payment_id: None,
    payment_completed_at: Utc::now(),
    expense_id: None,
    expense_name: String::from("The Good Stuff"),
  };

  payment_composite_test_1.create_payment(&mut main_store);

  let mut payment_composite_test_2 = PaymentComposite {
      account_id: None,
      account_name: String::from("new_bank"),
      amount_id: None,
      amount_standard: 5678.67,
      payment_id: None,
      payment_completed_at: Utc::now(),
      expense_id: None,
      expense_name: String::from("The Better Stuff"),
  };

  payment_composite_test_2.create_payment(&mut main_store);

  let mut payment_composite_test_3 = PaymentComposite {
      account_id: None,
      account_name: String::from("credit_union"),
      amount_id: None,
      amount_standard: 121212.34,
      payment_id: None,
      payment_completed_at: Utc::now(),
      expense_id: None,
      expense_name: String::from("The Best Stuff"),
  };

  payment_composite_test_3.create_payment(&mut main_store);

  main_store.write_to_csv(None);
}
