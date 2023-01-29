use crate::composite::payment_composite::PaymentComposite;
use crate::composite::payment_received_composite::PaymentReceivedComposite;
use chrono::NaiveDateTime;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PaymentEvent {
    pub event_type: String,
    pub name: String,
    pub account_name: String,
    pub amount: Decimal,
    pub completed_at: NaiveDateTime,
}

impl PaymentEvent {
    pub fn clone(&self) -> PaymentEvent {
        PaymentEvent {
            event_type: self.event_type.clone(),
            name: self.name.clone(),
            account_name: self.account_name.clone(),
            ..*self
        }
    }
}

pub enum PaymentEventComposite {
    P(PaymentComposite),
    PR(PaymentReceivedComposite),
    None,
}

impl PaymentEvent {
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
    use crate::storage::store::Store;
    use crate::test::spec::Spec;
    use chrono::NaiveDate;

    #[test]
    #[allow(non_snake_case)]
    fn to_composite__returns_payment_composite_when_0_is_payment() {
        let mut store = Store::new();
        Spec::init(&mut store);

        match (PaymentEvent {
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
