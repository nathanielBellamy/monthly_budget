use crate::composite::payment_composite::{PaymentComposite, PaymentCompositeStore};
use crate::composite::payment_event::{PaymentEvent, PaymentEventComposite};
use crate::composite::payment_received_composite::{
    PaymentReceivedComposite, PaymentReceivedCompositeStore,
};
use crate::storage::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    pub id: Option<usize>,
    pub payments: PaymentCompositeStore,
    pub payments_received: PaymentReceivedCompositeStore,
    pub date: NaiveDate,
}

impl CsvRecord<Day> for Day {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
        self.id = Some(new_id);
        self.id
    }

    fn clone_record(&self) -> Day {
        unimplemented!();
    }
}

impl CsvStore<Day> for Day {}

pub type DayStore = BTreeMap<usize, Day>;

impl Day {
    #[allow(unused)]
    pub fn new(year: i32, month: u32, day: u32) -> Day {
        Day {
            id: None,
            payments: PaymentCompositeStore::new(),
            payments_received: PaymentReceivedCompositeStore::new(),
            date: NaiveDate::from_ymd_opt(year, month, day).unwrap(),
        }
    }
    pub fn add_payment_event(&mut self, payment_event: &PaymentEvent) {
        match payment_event.to_composite() {
            PaymentEventComposite::P(pymnt_composite) => self.add_payment(pymnt_composite),
            PaymentEventComposite::PR(pymnt_rec_composite) => {
                self.add_payment_received(pymnt_rec_composite)
            }
            PaymentEventComposite::None => (),
        }
    }

    pub fn add_payment(&mut self, payment_comp: PaymentComposite) {
        PaymentComposite::save_to_store(payment_comp, &mut self.payments);
    }

    pub fn add_payment_received(&mut self, payment_rec_comp: PaymentReceivedComposite) {
        PaymentReceivedComposite::save_to_store(payment_rec_comp, &mut self.payments_received);
    }

    pub fn execute_payments_in_order(&mut self, store: &mut Store) -> Result<(), Box<dyn Error>> {
        let mut payment_times: Vec<(usize, NaiveDateTime, &str)> = vec![];
        for (id, pymnt) in self.payments.iter() {
            payment_times.push((*id, pymnt.payment_completed_at, "payment"));
        }

        for (id, pymnt_rec) in self.payments_received.iter() {
            payment_times.push((
                *id,
                pymnt_rec.payment_received_completed_at,
                "payment_received",
            ))
        }

        payment_times.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        for pymnt_event in payment_times.iter() {
            match pymnt_event.2 {
                "payment" => {
                    if let Entry::Occupied(mut record) = self.payments.entry(pymnt_event.0) {
                        record
                            .get_mut()
                            .create_payment(store, Some(pymnt_event.1))?;
                    }
                }
                "payment_received" => {
                    if let Entry::Occupied(mut record) = self.payments_received.entry(pymnt_event.0)
                    {
                        record
                            .get_mut()
                            .create_payment_received(store, Some(pymnt_event.1))?;
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod day_spec {

    use super::*;
    use crate::composite::payment_event::RecurrenceState;
    use crate::schema::account::Account;
    use crate::schema::account_balance::AccountBalance;
    use crate::schema::account_balance::AccountBalanceStore;
    use crate::test::spec::Spec;
    use rust_decimal::Decimal;

    #[test]
    #[allow(non_snake_case)]
    fn add_payment_event__adds_payment_when_event_0_is_payment() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut day = Day::new(2023, 6, 6);
        let payment_event = PaymentEvent {
            id: None,
            event_type: "payment".to_string(),
            name: "My Payment".to_string(),
            account_name: "Big Bank".to_string(),
            amount: Decimal::new(12345, 2),
            completed_at: NaiveDate::from_ymd_opt(2023, 6, 6)
                .unwrap()
                .and_hms_opt(12, 00, 00)
                .unwrap(),
            recurrence_state: RecurrenceState::None,
        };

        assert_eq!(0, day.payments.len());
        day.add_payment_event(&payment_event);
        assert_eq!(1, day.payments.len());
    }

    #[test]
    #[allow(non_snake_case)]
    fn add_payment_event__adds_payment_received_when_event_0_is_payment_received() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut day = Day::new(2023, 6, 6);
        let payment_event = PaymentEvent {
            id: None,
            event_type: "payment_received".to_string(),
            name: "My Payment Received".to_string(),
            account_name: "Big Bank".to_string(),
            amount: Decimal::new(12345, 2),
            completed_at: NaiveDate::from_ymd_opt(2023, 6, 6)
                .unwrap()
                .and_hms_opt(12, 00, 00)
                .unwrap(),
            recurrence_state: RecurrenceState::None,
        };

        assert_eq!(0, day.payments_received.len());
        day.add_payment_event(&payment_event);
        assert_eq!(1, day.payments_received.len());
    }

    #[test]
    #[allow(non_snake_case)]
    fn execute_payments_in_order__enacts_all_payment_events_on_day_in_chrono_order() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut day = Day::new(2023, 6, 6);
        day.add_payment_event(&PaymentEvent {
            id: None,
            event_type: "payment".to_string(),
            name: "My Payment".to_string(),
            account_name: "New Bank".to_string(),
            amount: Decimal::new(001, 2),
            completed_at: NaiveDate::from_ymd_opt(2023, 6, 6)
                .unwrap()
                .and_hms_opt(12, 00, 01)
                .unwrap(),
            recurrence_state: RecurrenceState::None,
        });
        day.add_payment_event(&PaymentEvent {
            id: None,
            event_type: "payment_received".to_string(),
            name: "My Payment Received".to_string(),
            account_name: "New Bank".to_string(),
            amount: Decimal::new(010, 2),
            completed_at: NaiveDate::from_ymd_opt(2023, 6, 6)
                .unwrap()
                .and_hms_opt(12, 00, 02)
                .unwrap(),
            recurrence_state: RecurrenceState::None,
        });
        day.add_payment_event(&PaymentEvent {
            id: None,
            event_type: "payment".to_string(),
            name: "My Payment".to_string(),
            account_name: "New Bank".to_string(),
            amount: Decimal::new(100, 2),
            completed_at: NaiveDate::from_ymd_opt(2023, 6, 6)
                .unwrap()
                .and_hms_opt(12, 00, 03)
                .unwrap(),
            recurrence_state: RecurrenceState::None,
        });
        day.add_payment_event(&PaymentEvent {
            id: None,
            event_type: "payment_received".to_string(),
            name: "My Payment Received".to_string(),
            account_name: "New Bank".to_string(),
            amount: Decimal::new(1000, 2),
            completed_at: NaiveDate::from_ymd_opt(2023, 6, 6)
                .unwrap()
                .and_hms_opt(12, 00, 04)
                .unwrap(),
            recurrence_state: RecurrenceState::None,
        });

        assert_eq!(2, day.payments.len());
        assert_eq!(2, day.payments_received.len());

        day.execute_payments_in_order(&mut store).unwrap();

        let account = Account::by_name("New Bank", &mut store.accounts).unwrap();
        let account_balance_ids = account.account_balance_ids(&mut store.account_balances);

        let mut acc_bal_store = AccountBalanceStore::new();
        for acc_bal_id in account_balance_ids.iter() {
            let mut acc_bal = store.account_balances[acc_bal_id].clone_record();
            acc_bal.id = None;
            AccountBalance::save_to_store(acc_bal, &mut acc_bal_store);
        }

        assert_eq!(Decimal::new(-0001, 2), acc_bal_store[&1].amount); // pay .01
        assert_eq!(Decimal::new(009, 2), acc_bal_store[&2].amount); // receive .1
        assert_eq!(Decimal::new(-091, 2), acc_bal_store[&3].amount); // pay 1
        assert_eq!(Decimal::new(909, 2), acc_bal_store[&4].amount); // receive 10
    }
}
