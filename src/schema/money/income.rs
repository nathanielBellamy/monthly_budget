use crate::schema::money::payment_received::PaymentReceived;
use crate::store::store::Store;
use crate::traits::csv_store::CsvStore;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Income {
    pub id: usize,
    pub active: bool,
    pub name: String,
}

impl CsvStore for Income {}

impl<'a, 'b: 'a> Income {
    pub fn payments_received(&'a self, store: &'b Store) -> Vec<&PaymentReceived> {
        let mut payments_received: Vec<&PaymentReceived> = vec![];
        for payment_received in store.payments_received.iter() {
            if payment_received.income_id == self.id {
                payments_received.push(payment_received)
            }
        }
        payments_received
    }

    pub fn last_payment_received(&'a self, store: &'b Store) -> Option<&PaymentReceived> {
        for payment_received in store.payments_received.iter().rev() {
            if payment_received.income_id == self.id {
                return Some(payment_received);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn payments_received__retrieves_records_from_store() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let income = &store.incomes[0];
        let payments_received = income.payments_received(&store);
        let first_payment_received: &PaymentReceived = payments_received[0];
        assert_eq!(first_payment_received.id, 1);
        assert_eq!(
            first_payment_received.completed_at,
            DateTime::parse_from_str("2023-01-01 11:11:11-08:00", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
        assert_eq!(first_payment_received.income_id, income.id);
        assert_eq!(first_payment_received.amount_id, 2);

        let second_payment_received: &PaymentReceived = payments_received[1];
        assert_eq!(second_payment_received.id, 3);
        assert_eq!(second_payment_received.income_id, income.id);
        assert_eq!(
            second_payment_received.completed_at,
            DateTime::parse_from_str("2023-01-03 13:13:13-08:00", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
        assert_eq!(second_payment_received.amount_id, 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn last_payment__returns_most_recent_payment() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let income = &store.incomes[0];
        let payments_received = income.payments_received(&store);
        let most_recent_payment_received: &PaymentReceived = payments_received[1];
        assert_eq!(
            payments_received[1].completed_at > payments_received[0].completed_at,
            true
        );

        let res: &PaymentReceived = income.last_payment_received(&store).unwrap();
        assert_eq!(res.id, most_recent_payment_received.id);
        assert_eq!(res.completed_at, most_recent_payment_received.completed_at);
        assert_eq!(res.income_id, income.id);
        assert_eq!(res.amount_id, most_recent_payment_received.amount_id);
    }
}
