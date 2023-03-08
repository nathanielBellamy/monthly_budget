use crate::calendar::calendar_slice::CalendarSlice;
use crate::calendar::month::Month;
use crate::calendar::year_month::YearMonth as YM;
use crate::composite::payment_composite::PaymentComposite;
use crate::composite::payment_received_composite::PaymentReceivedComposite;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::{Datelike, NaiveDateTime};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentEvent {
    pub id: Option<usize>,
    pub event_type: String,
    pub name: String,
    pub account_name: String,
    pub amount: Decimal,
    pub completed_at: NaiveDateTime,
    pub recurrence_state: RecurrenceState,
}

pub enum PaymentEventComposite {
    P(PaymentComposite),
    PR(PaymentReceivedComposite),
    None,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Default)]
#[serde(tag = "rs")]
pub enum RecurrenceState {
    First, // mark Income/Expense active
    Active,
    Last, // marke Income/Expense inactive
    #[default]
    None, // Not a recurring payment event
}

// fetch
type PaymentEventFetchResult = Result<Vec<PaymentEvent>, Box<dyn Error>>;
// store
pub type PaymentEventStore = BTreeMap<usize, PaymentEvent>;
// bin
pub type PaymentEventBinStore = BTreeMap<YM, PaymentEventStore>;
type PaymentEventBinResult = Result<(), Box<dyn Error>>;

impl CsvRecord<PaymentEvent> for PaymentEvent {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
        self.id = Some(new_id);
        self.id
    }

    fn clone_record(&self) -> PaymentEvent {
        PaymentEvent {
            event_type: self.event_type.clone(),
            name: self.name.clone(),
            account_name: self.account_name.clone(),
            ..*self
        }
    }
}

impl CsvStore<PaymentEvent> for PaymentEvent {}

impl PaymentEvent {
    pub fn fetch_events(path: String) -> PaymentEventFetchResult {
        let data: String = fs::read_to_string(path)?.parse()?;
        let payment_events: Vec<PaymentEvent> = serde_json::from_str(&data)?;
        Ok(payment_events)
    }

    pub fn fetch_and_bin_one_off_events(
        path: String,
        cal_slice: &CalendarSlice,
        bin_store: &mut PaymentEventBinStore,
    ) -> PaymentEventBinResult {
        let payment_events = PaymentEvent::fetch_events(path)?;
        for payment_event in payment_events.into_iter() {
            let ym = YM::new(
                payment_event.completed_at.year(),
                Month::key_from_id(payment_event.completed_at.month()),
            );
            // YearMonth impl Eq, PartialEq, PartialOrd, Ord
            if cal_slice.start <= ym && ym <= cal_slice.end {
                let store = bin_store.entry(ym).or_default();
                PaymentEvent::save_to_store(payment_event, store);
            }
        }
        Ok(())
    }

    pub fn to_composite(&self) -> PaymentEventComposite {
        match self.event_type.as_str() {
            "payment" => PaymentEventComposite::P(PaymentComposite {
                id: None,
                account_id: None,
                account_name: self.account_name.clone(),
                account_balance_id: None,
                prev_balance: None,
                ending_balance: None,
                amount_id: None,
                amount_standard: self.amount,
                payment_id: None,
                payment_completed_at: self.completed_at,
                expense_id: None,
                expense_name: self.name.clone(),
                recurrence_state: self.recurrence_state,
            }),
            "payment_received" => PaymentEventComposite::PR(PaymentReceivedComposite {
                id: None,
                account_id: None,
                account_name: self.account_name.clone(),
                account_balance_id: None,
                prev_balance: None,
                ending_balance: None,
                amount_id: None,
                amount_standard: self.amount,
                payment_received_id: None,
                payment_received_completed_at: self.completed_at,
                income_id: None,
                income_name: self.name.clone(),
                recurrence_state: self.recurrence_state,
            }),
            _ => PaymentEventComposite::None,
        }
    }
}

#[cfg(test)]
mod expense_spec {
    use super::*;
    use crate::calendar::month_key::MonthKey as MK;
    use crate::storage::store::Store;
    use crate::test::spec::Spec;
    use chrono::NaiveDate;

    fn json_path() -> String {
        "src/test/data/events/one_off.json".to_string()
    }

    #[test]
    #[allow(non_snake_case)]
    fn fetch_events__parses_json_into_vec_of_payment_events() {
        let payment_events: Vec<PaymentEvent> = PaymentEvent::fetch_events(json_path()).unwrap();
        assert_eq!(payment_events.len(), 4);
        let event = &payment_events[0];
        assert_eq!(event.event_type, "payment".to_string());
        assert_eq!(event.name, "Tollbooth".to_string());
        assert_eq!(event.account_name, "Credit Union".to_string());
        assert_eq!(event.amount, Decimal::new(20, 0));
        assert_eq!(
            event.completed_at,
            NaiveDate::from_ymd_opt(2023, 2, 25)
                .unwrap()
                .and_hms_opt(12, 00, 00)
                .unwrap()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn fetch_and_bin_events_by_month__returns_PaymentEventBinStore_populated_by_payment_events() {
        let cal_slice = CalendarSlice::new(YM::new(2023, MK::Feb), YM::new(2024, MK::Mar)).unwrap();
        let mut bin_store = PaymentEventBinStore::new();
        PaymentEvent::fetch_and_bin_one_off_events(json_path(), &cal_slice, &mut bin_store)
            .unwrap();
        assert_eq!(bin_store.len(), 3);

        let mar_store = bin_store
            .entry(YM::new(2023_i32, MK::Mar))
            .or_insert(PaymentEventStore::new());
        assert_eq!(mar_store.len(), 2);

        let mar_event = &mar_store[&1];
        assert_eq!(mar_event.event_type, "payment");
        assert_eq!(mar_event.name, "Car Repair");
        assert_eq!(mar_event.account_name, "Credit Union");
        assert_eq!(mar_event.amount, Decimal::new(400, 0));
        assert_eq!(
            mar_event.completed_at,
            NaiveDate::from_ymd_opt(2023, 3, 12)
                .unwrap()
                .and_hms_opt(15, 00, 00)
                .unwrap()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn fetch_and_bin_events_by_month__adds_only_those_payment_events_between_start_and_end() {
        let cal_slice = CalendarSlice::new(YM::new(2023, MK::Mar), YM::new(2023, MK::Jun)).unwrap();
        let mut bin_store = PaymentEventBinStore::new();
        PaymentEvent::fetch_and_bin_one_off_events(json_path(), &cal_slice, &mut bin_store)
            .unwrap();
        assert_eq!(bin_store.len(), 2);

        let mar_store = bin_store
            .entry(YM::new(2023_i32, MK::Mar))
            .or_insert(PaymentEventStore::new());
        assert_eq!(mar_store.len(), 2);
    }

    #[test]
    #[allow(non_snake_case)]
    fn to_composite__returns_payment_composite_when_0_is_payment() {
        let mut store = Store::new();
        Spec::init(&mut store);

        match (PaymentEvent {
            id: None,
            event_type: "payment".to_string(),
            name: "My Payment".to_string(),
            account_name: "My Bank Account".to_string(),
            amount: Decimal::new(123456, 2),
            completed_at: NaiveDate::from_ymd_opt(2023, 1, 2)
                .unwrap()
                .and_hms_opt(12, 00, 00)
                .unwrap(),
            recurrence_state: RecurrenceState::None,
        })
        .to_composite()
        {
            PaymentEventComposite::P(payment) => {
                assert_eq!(payment.expense_name, "My Payment".to_string())
            }
            _ => assert_eq!(0, 1),
        };
    }

    #[test]
    #[allow(non_snake_case)]
    fn to_composite__returns_payment_received_composite_when_0_is_payment_received() {
        let mut store = Store::new();
        Spec::init(&mut store);

        match (PaymentEvent {
            id: None,
            event_type: "payment_received".to_string(),
            name: "My Payment Received".to_string(),
            account_name: "My Bank Account".to_string(),
            amount: Decimal::new(123456, 2),
            completed_at: NaiveDate::from_ymd_opt(2023, 1, 2)
                .unwrap()
                .and_hms_opt(12, 00, 00)
                .unwrap(),
            recurrence_state: RecurrenceState::None,
        })
        .to_composite()
        {
            PaymentEventComposite::PR(payment) => {
                assert_eq!(payment.income_name, "My Payment Received".to_string())
            }
            _ => assert_eq!(0, 1),
        };
    }
}
