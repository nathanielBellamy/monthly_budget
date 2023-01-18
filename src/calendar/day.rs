use crate::schema::payment::Payment;
use crate::schema::payment_received::PaymentReceived;
use crate::store::store::Store;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    pub payments: Vec<Payment>,
    pub payments_received: Vec<PaymentReceived>,
    pub date: DateTime<Local>,
}

impl Day {
    pub fn add_payment(&mut self, payment: Payment) -> () {
        self.payments.push(payment)
    }

    // pub fn total_spent(&self) -> f64 {
    //     let mut total: f64 = 0.0;
    //     for payment in self.payments.iter() {
    //         total += payment.amount()
    //     }
    // }

    pub fn add_payment_received(&mut self, payment_received: PaymentReceived) -> () {
        self.payments_received.push(payment_received)
    }
}

#[cfg(test)]
mod day_spec {
    use super::*;
    use crate::spec::spec::Spec;
    use crate::traits::csv_store::CsvStore;

    #[test]
    #[allow(non_snake_case)]
    fn add_payment__adds_payment_to_self_payments() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment: Payment = Payment::by_id(1, &mut store.payments).unwrap();
        let mut day = Day {
            payments: vec![],
            payments_received: vec![],
            date: Local::now(),
        };
        day.add_payment(payment);
        assert_eq!(day.payments.len(), 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn add_payment_received__adds_payment_received_to_self_payments_received() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let payment_rec = PaymentReceived::by_id(1, &mut store.payments_received).unwrap();
        let mut day = Day {
            payments: vec![],
            payments_received: vec![],
            date: Local::now(),
        };
        day.add_payment_received(payment_rec);
        assert_eq!(day.payments_received.len(), 1);
    }
}
