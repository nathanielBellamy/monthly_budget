use crate::calendar::month::Month;
use crate::calendar::year_month::YearMonth;
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
}

pub enum PaymentEventComposite {
    P(PaymentComposite),
    PR(PaymentReceivedComposite),
    None,
}

type PaymentEventFetchResult = Result<Vec<PaymentEvent>, Box<dyn Error>>;
type PaymentEventBinResult = Result<PaymentEventBinStore, Box<dyn Error>>;

pub type PaymentEventBinStore = BTreeMap<YearMonth, PaymentEventStore>;
pub type PaymentEventStore = BTreeMap<usize, PaymentEvent>;

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

    pub fn fetch_and_bin_events_by_month(path: String) -> PaymentEventBinResult {
        let mut bin_store = PaymentEventBinStore::new();
        let payment_events = PaymentEvent::fetch_events(path)?;
        for payment_event in payment_events.into_iter() {
            let year = payment_event.completed_at.year();
            let month = payment_event.completed_at.month();
            let store = bin_store
                .entry(YearMonth(year, Month::key_from_id(month)))
                .or_default();
            PaymentEvent::save_to_store(payment_event, store);
        }
        Ok(bin_store)
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
            }),
            _ => PaymentEventComposite::None,
        }
    }
}

#[cfg(test)]
mod expense_spec {
    use super::*;
    use crate::calendar::month::MonthKey;
    use crate::storage::store::Store;
    use crate::test::spec::Spec;
    use chrono::NaiveDate;

    fn json_path() -> String {
        "src/test/data/json/payment_events.json".to_string()
    }

    #[test]
    #[allow(non_snake_case)]
    fn fetch_events__parses_json_into_vec_of_payment_events() {
        let payment_events: Vec<PaymentEvent> = PaymentEvent::fetch_events(json_path()).unwrap();
        assert_eq!(payment_events.len(), 6);
        let event = &payment_events[0];
        assert_eq!(event.event_type, "payment".to_string());
        assert_eq!(event.name, "food".to_string());
        assert_eq!(event.account_name, "piggybank".to_string());
        assert_eq!(event.amount, Decimal::new(1250, 1));
        assert_eq!(
            event.completed_at,
            NaiveDate::from_ymd_opt(2023, 2, 20)
                .unwrap()
                .and_hms_opt(12, 00, 00)
                .unwrap()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn fetch_and_bin_events_by_month__returns_PaymentEventBinStore_populated_by_payment_events() {
        let mut bin_store = PaymentEvent::fetch_and_bin_events_by_month(json_path()).unwrap();
        assert_eq!(bin_store.len(), 2);

        let feb_store = bin_store
            .entry(YearMonth(2023_i32, MonthKey::Feb))
            .or_insert(PaymentEventStore::new());
        assert_eq!(feb_store.len(), 3);

        let feb_event = &feb_store[&1];
        assert_eq!(feb_event.event_type, "payment");
        assert_eq!(feb_event.name, "food");
        assert_eq!(feb_event.account_name, "piggybank");
        assert_eq!(feb_event.amount, Decimal::new(1250, 1));
        assert_eq!(
            feb_event.completed_at,
            NaiveDate::from_ymd_opt(2023, 2, 20)
                .unwrap()
                .and_hms_opt(12, 00, 00)
                .unwrap()
        );
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
