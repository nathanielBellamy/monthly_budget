use crate::schema::payment::{Payment, PaymentStore};
use crate::storage::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Expense {
    pub id: Option<usize>,
    pub active: bool,
    pub name: String,
}

impl CsvRecord<Expense> for Expense {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
        self.id = Some(new_id);
        self.id
    }

    fn clone_record(&self) -> Expense {
        Expense {
            name: self.name.clone(),
            ..*self
        }
    }
}
impl CsvStore<Expense> for Expense {}

pub type ExpenseStore = BTreeMap<usize, Expense>;

impl<'a, 'b: 'a> Expense {
    pub fn by_name(name: &'a str, store: &'b ExpenseStore) -> Option<Expense> {
        let mut expense: Option<Expense> = None;
        for (_id, exp) in store.iter() {
            if exp.name == name {
                expense = Some(exp.clone_record());
                break;
            }
        }
        expense
    }

    pub fn name_by_id(id: usize, store: &mut Store) -> String {
        match Expense::by_id(id, &mut store.expenses) {
            None => format!("No Name Found for Expense Id: {id}"),
            Some(expense) => expense.name,
        }
    }

    pub fn total_by_id(id: usize, store: &mut Store) -> Decimal {
        match Expense::by_id(id, &mut store.expenses) {
            None => Decimal::new(00, 1),
            Some(expense) => {
                let payments = expense.payments(&mut store.payments);
                Payment::total(payments, &store.amounts)
            }
        }
    }

    pub fn payments(&'a self, store: &'b mut PaymentStore) -> PaymentStore {
        let mut payments: PaymentStore = BTreeMap::new();
        for (id, payment) in store.iter() {
            if payment.expense_id == self.id.unwrap() {
                payments
                    .entry(*id)
                    .or_insert_with(|| payment.clone_record());
            }
        }
        payments
    }

    #[allow(unused)]
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
    use crate::test::spec::Spec;
    use chrono::NaiveDateTime;

    #[test]
    #[allow(non_snake_case)]
    fn by_name__retrieves_expense_from_store_by_name() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let expense = Expense::by_name("mortgage", &mut store.expenses).unwrap();
        assert_eq!(1, expense.id.unwrap());
    }

    #[test]
    #[allow(non_snake_case)]
    fn name_by_id__retrieves_expense_from_store_by_name() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let name = Expense::name_by_id(1, &mut store);
        assert_eq!("mortgage".to_string(), name);
    }

    #[test]
    #[allow(non_snake_case)]
    fn payments__retrieves_records_from_store() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let expense = Expense::by_id(1, &mut store.expenses).unwrap();
        let payments = expense.payments(&mut store.payments);
        let first_payment: Payment = payments[&1].clone_record();
        let second_payment: Payment = payments[&4].clone_record();
        assert_eq!(first_payment.id.unwrap(), 1);
        assert_eq!(
            first_payment.completed_at,
            NaiveDateTime::parse_from_str("2023-01-01 11:11:11-08:00", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
        );
        assert_eq!(first_payment.expense_id, expense.id.unwrap());
        assert_eq!(first_payment.amount_id, 1);

        assert_eq!(second_payment.id.unwrap(), 4);
        assert_eq!(second_payment.expense_id, expense.id.unwrap());
        assert_eq!(
            second_payment.completed_at,
            NaiveDateTime::parse_from_str("2023-01-04 14:14:14-08:00", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
        );
        assert_eq!(second_payment.amount_id, 1);
    }

    #[test]
    #[allow(non_snake_case)]
    fn last_payment__returns_most_recent_payment() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let expense = Expense::by_id(1, &mut store.expenses).unwrap();
        let res: Payment = expense.last_payment(&mut store.payments).unwrap();

        assert_eq!(res.id.unwrap(), 4);
        assert_eq!(
            res.completed_at,
            NaiveDateTime::parse_from_str("2023-01-04 14:14:14-08:00", "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
        );
    }
}
