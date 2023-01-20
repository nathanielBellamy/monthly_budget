use crate::error_handler::error_handler::ErrorHandler;
use crate::schema::payment_received::{PaymentReceived, PaymentReceivedStore};
use crate::store::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Income {
    pub id: Option<usize>,
    pub active: bool,
    pub name: String,
}

impl CsvRecord<Income> for Income {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
      self.id = Some(new_id);
      self.id
    }

    fn clone_record(&self) -> Income {
        Income {
            name: self.name.clone(),
            ..*self
        }
    }
}
impl CsvStore<Income> for Income {}

pub type IncomeStore = HashMap<usize, Income>;

impl<'a, 'b: 'a> Income {
    pub fn payments_received(&'a self, store: &'b PaymentReceivedStore) -> PaymentReceivedStore {
        if let None = self.id {
          ErrorHandler::log(From::from(format!("Income {:?} does not exist in main_store.", self.name)))
        }

        let mut payments_received: PaymentReceivedStore = HashMap::new();
        for (id, payment_received) in store.iter() {
            if payment_received.income_id == self.id.unwrap() { // TODO: handle error
                payments_received
                    .entry(*id)
                    .or_insert(payment_received.clone_record());
            }
        }
        println!("HERHEHEREHRERH: {:?}", payments_received);
        payments_received
    }

    pub fn last_payment_received(
        &'a self,
        store: &'b PaymentReceivedStore,
    ) -> Option<PaymentReceived> {
        let mut payment_received: Option<PaymentReceived> = None;
        for (_id, pymnt_rcvd) in self.payments_received(store).iter() {
            match payment_received {
                None => payment_received = Some(*pymnt_rcvd), // set first
                Some(pr) => {
                    if pymnt_rcvd.completed_at > pr.completed_at {
                        payment_received = Some(pymnt_rcvd.clone_record()) //TODO: refactor to avoid unecessayr clone
                    }
                }
            }
        }
        payment_received
    }
}

#[cfg(test)]
mod income_spec {
    use super::*;
    use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn payments_received__retrieves_records_from_store() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let income = Income::by_id(1, &mut store.incomes).unwrap();
        let payments_received = income.payments_received(&store.payments_received);
        let first_payment_received: PaymentReceived = payments_received[&1];
        assert_eq!(first_payment_received.id.unwrap(), 1);
        assert_eq!(
            first_payment_received.completed_at,
            DateTime::parse_from_str("2023-01-01 11:11:11-08:00", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
        assert_eq!(first_payment_received.income_id, income.id.unwrap());
        assert_eq!(first_payment_received.amount_id, 2);

        let second_payment_received: PaymentReceived = payments_received[&3];
        assert_eq!(second_payment_received.id.unwrap(), 3);
        assert_eq!(second_payment_received.income_id, income.id.unwrap());
        assert_eq!(
            second_payment_received.completed_at,
            DateTime::parse_from_str("2023-01-03 13:13:13-08:00", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
        assert_eq!(second_payment_received.amount_id, 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn last_payment_received__returns_most_recent_payment() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let income = Income::by_id(1, &mut store.incomes).unwrap();
        let payments_received = income.payments_received(&store.payments_received);
        let most_recent_payment_received: PaymentReceived = payments_received[&3]; // by construction
        assert_eq!(
            payments_received[&3].completed_at > payments_received[&1].completed_at,
            true
        );

        let res: PaymentReceived = income
            .last_payment_received(&store.payments_received)
            .unwrap();
        assert_eq!(res.id, most_recent_payment_received.id);
        assert_eq!(res.completed_at, most_recent_payment_received.completed_at);
        assert_eq!(res.income_id, income.id.unwrap());
        assert_eq!(res.amount_id, most_recent_payment_received.amount_id);
    }
}