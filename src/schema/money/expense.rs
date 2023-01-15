use crate::schema::money::payment::Payment;
use crate::store::store::Store;
use crate::traits::csv_store::CsvStore;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub id: usize,
    pub active: bool,
    pub name: String,
}

impl CsvStore for Expense {}

impl<'a, 'b: 'a> Expense {
    pub fn payments(&'a self, store: &'b Store) -> Vec<&Payment> {
        let mut payments: Vec<&Payment> = vec![];
        for payment in store.payments.iter() {
            if payment.expense_id == self.id {
                payments.push(payment)
            }
        }
        payments
    }

    pub fn last_payment(&'a self, store: &'b Store) -> Option<&Payment> {
        for payment in store.payments.iter().rev() {
            if payment.expense_id == self.id {
                return Some(payment);
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
    fn payments__retrieves_records_from_store() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let expense = &store.expenses[0];
        let payments = expense.payments(&store);
        let first_payment: &Payment = payments[0];
        assert_eq!(first_payment.id, 1);
        assert_eq!(
            first_payment.completed_at,
            DateTime::parse_from_str("2023-01-01 11:11:11-08:00", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
        assert_eq!(first_payment.expense_id, expense.id);
        assert_eq!(first_payment.amount_id, 1);

        let second_payment: &Payment = payments[1];
        assert_eq!(second_payment.id, 4);
        assert_eq!(second_payment.expense_id, expense.id);
        assert_eq!(
            second_payment.completed_at,
            DateTime::parse_from_str("2023-01-04 14:14:14-08:00", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
        assert_eq!(second_payment.amount_id, 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn last_payment__returns_most_recent_payment() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let expense = &store.expenses[0];
        let payments = expense.payments(&store);
        let most_recent_payment: &Payment = payments[1];
        assert_eq!(payments[1].completed_at > payments[0].completed_at, true);

        let res: &Payment = expense.last_payment(&store).unwrap();
        assert_eq!(res.id, most_recent_payment.id);
        assert_eq!(res.completed_at, most_recent_payment.completed_at);
        assert_eq!(res.expense_id, expense.id);
        assert_eq!(res.amount_id, most_recent_payment.amount_id);
    }
}
