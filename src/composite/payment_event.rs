use crate::composite::payment_composite::PaymentComposite;
use crate::composite::payment_received_composite::PaymentReceivedComposite;
use chrono::NaiveDateTime;
use rust_decimal::prelude::*;

#[derive(Debug)]
pub struct PaymentEvent(
    // tuple struct used for generality
    pub &'static str,  // type 0
    pub String,        // name 1
    pub String,        // acc_name 2
    pub Decimal,       // amount 3
    pub NaiveDateTime, // completed_at 4
);

impl PaymentEvent {
    pub fn clone(&self) -> PaymentEvent {
        PaymentEvent(
            self.0,
            self.1.clone(),
            self.2.clone(),
            self.3,
            self.4,
        )
    }
}

pub enum PaymentEventComposite {
    P(PaymentComposite),
    PR(PaymentReceivedComposite),
    None,
}

impl PaymentEvent {
    pub fn to_composite(&self) -> PaymentEventComposite {
        match self.0 {
            "payment" => PaymentEventComposite::P(PaymentComposite {
                id: None,
                account_id: None,
                account_name: self.2.clone(),
                account_balance_id: None,
                prev_balance: None,
                ending_balance: None,
                amount_id: None,
                amount_standard: self.3,
                payment_id: None,
                payment_completed_at: self.4,
                expense_id: None,
                expense_name: self.1.clone(),
            }),
            "payment_received" => PaymentEventComposite::PR(PaymentReceivedComposite {
                id: None,
                account_id: None,
                account_name: self.2.clone(),
                account_balance_id: None,
                prev_balance: None,
                ending_balance: None,
                amount_id: None,
                amount_standard: self.3,
                payment_received_id: None,
                payment_received_completed_at: self.4,
                income_id: None,
                income_name: self.1.clone(),
            }),
            _ => PaymentEventComposite::None,
        }
    }
}

#[cfg(test)]
mod expense_spec {
    use super::*;
    use crate::test::spec::Spec;
    use crate::storage::store::Store;
    use chrono::NaiveDate;

    #[test]
    #[allow(non_snake_case)]
    fn to_composite__returns_payment_composite_when_0_is_payment() {
        let mut store = Store::new();
        Spec::init(&mut store);

        match PaymentEvent(
            "payment",
            "My Payment".to_string(),
            "My Bank Account".to_string(),
            Decimal::new(123456, 2),
            NaiveDate::from_ymd_opt(2023, 1, 2)
                .unwrap()
                .and_hms_opt(12, 00, 00)
                .unwrap(),
        )
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

        match PaymentEvent(
            "payment_received",
            "My Payment Received".to_string(),
            "My Bank Account".to_string(),
            Decimal::new(123456, 2),
            NaiveDate::from_ymd_opt(2023, 1, 2)
                .unwrap()
                .and_hms_opt(12, 00, 00)
                .unwrap(),
        )
        .to_composite()
        {
            PaymentEventComposite::PR(payment) => {
                assert_eq!(payment.income_name, "My Payment Received".to_string())
            }
            _ => assert_eq!(0, 1),
        };
    }
}
