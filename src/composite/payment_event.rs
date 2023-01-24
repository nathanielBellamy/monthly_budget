use chrono::NaiveDateTime;
use crate::composite::payment_composite::PaymentComposite;
use crate::composite::payment_received_composite::PaymentReceivedComposite;

#[derive(Debug)]
pub struct PaymentEvent( // tuple struct used for generality
  pub &'static str, // type 0
  pub String, // name 1
  pub String, // acc_name 2
  pub f64, // amount 3
  pub NaiveDateTime, // completed_at 4
);

impl PaymentEvent {
  pub fn clone(&self) -> PaymentEvent {
    PaymentEvent(
      self.0,
      self.1.clone(),
      self.2.clone(),
      self.3,
      self.4.clone(),
    )
  }
}

pub enum PaymentEventComposite {
  P(PaymentComposite),
  PR(PaymentReceivedComposite),
  None
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
        _ => PaymentEventComposite::None
      }
    }
}
