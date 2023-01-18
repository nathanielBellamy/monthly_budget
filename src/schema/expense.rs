use crate::schema::payment::{Payment, PaymentStore};
use crate::store::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
// use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    pub id: usize,
    pub active: bool,
    pub name: String,
}

impl CsvRecord<Expense> for Expense {
    fn id(&self) -> usize {
        self.id
    }

    fn clone_record(&self) -> Expense {
        Expense {
            name: self.name.clone(),
            ..*self
        }
    }
}
impl CsvStore for Expense {}

pub type ExpenseStore = HashMap<usize, Expense>;

impl<'a, 'b: 'a> Expense {
    pub fn payments(&'a self, store: &'b mut PaymentStore) -> PaymentStore {
        let mut payments: PaymentStore = HashMap::new();
        for (id, payment) in store.iter() {
            if *id == self.id {
                payments.entry(*id).or_insert(payment.clone_record());
            }
        }
        payments
    }

    pub fn last_payment(&'a self, store: &'b mut PaymentStore) -> Option<Payment> {
        let mut last_payment: Option<Payment> = None;
        for (_id, payment) in self.payments(store).iter() {
            match last_payment {
                None => last_payment = Some(*payment), // set first
                Some(lst_pymnt_so_far) => {
                    if payment.completed_at > lst_pymnt_so_far.completed_at {
                        last_payment = Some(*payment)
                    }
                }
            }
        }
        last_payment
    }
}

#[cfg(test)]
mod expense_spec {
    use super::*;
    use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn payments__retrieves_records_from_store() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let expense = Expense::by_id(1, &mut store.expenses).unwrap();
        let payments = expense.payments(&mut store.payments);
        let first_payment: Payment = payments[0].clone_record();
        let second_payment: Payment = payments[1].clone_record();
        assert_eq!(first_payment.id, 1);
        assert_eq!(
            first_payment.completed_at,
            DateTime::parse_from_str("2023-01-01 11:11:11-08:00", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
        assert_eq!(first_payment.expense_id, expense.id);
        assert_eq!(first_payment.amount_id, 1);

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

        let expense = Expense::by_id(1, &mut store.expenses).unwrap();
        let res: Payment = expense.last_payment(&mut store).unwrap();

        assert_eq!(res.id, 4);
        assert_eq!(
            res.completed_at,
            DateTime::parse_from_str("2023-01-04 14:14:14-08:00", "%Y-%m-%d %H:%M:%S %z").unwrap()
        );
    }
}
