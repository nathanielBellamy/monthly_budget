use crate::composite::payment_display::PaymentDisplay;
use crate::error::error_handler::ErrorHandler;
use crate::schema::account::Account;
use crate::schema::account_balance::AccountBalance;
use crate::schema::amount::Amount;
use crate::schema::income::Income;
use crate::schema::payment_received::PaymentReceived;
use crate::storage::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::{NaiveDateTime, Utc};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentReceivedComposite {
    pub id: Option<usize>,
    pub account_id: Option<usize>,
    pub account_name: String,
    pub account_balance_id: Option<usize>, // id of account_balance resulting from creation of payment_received
    #[serde(with = "rust_decimal::serde::float_option")]
    pub prev_balance: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::float_option")]
    pub ending_balance: Option<Decimal>,
    pub amount_id: Option<usize>,
    #[serde(with = "rust_decimal::serde::float")]
    pub amount_standard: Decimal,
    pub payment_received_id: Option<usize>,
    pub payment_received_completed_at: NaiveDateTime,
    pub income_id: Option<usize>,
    pub income_name: String,
}

impl CsvRecord<PaymentReceivedComposite> for PaymentReceivedComposite {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
        self.id = Some(new_id);
        self.id
    }

    fn clone_record(&self) -> PaymentReceivedComposite {
        self.clone()
    }
}

impl CsvStore<PaymentReceivedComposite> for PaymentReceivedComposite {}

pub type PaymentReceivedCompositeStore = BTreeMap<usize, PaymentReceivedComposite>;

type CreatePaymentReceivedResult = Result<(), Box<dyn Error>>;

impl PaymentReceivedComposite {
    pub fn display(&self) -> PaymentDisplay {
        PaymentDisplay {
            id: self.id,
            name: self.income_name.clone(),
            amount: self.amount_standard,
            account_name: self.account_name.clone(),
            completed_at: self.payment_received_completed_at,
            prev_balance: self.prev_balance,
            ending_balance: self.ending_balance,
        }
    }

    pub fn create_payment_received(
        &mut self,
        store: &mut Store,
        complete_at: Option<NaiveDateTime>,
    ) -> CreatePaymentReceivedResult {
        if let Some(id) = self.payment_received_id {
            ErrorHandler::log(From::from(format!("PaymentReceived {id} already exists.")))
        }

        if self.account_id.is_none() {
            // try lookup by name
            match Account::by_name(&self.account_name, &store.accounts) {
                None => {
                    // create Account record
                    self.account_id = Some(Account::new_id(&store.accounts));
                    self.account_id = Some(Account::save_to_store(
                        Account {
                            id: self.account_id, // T::new_id returns T, this unwrap is a formality
                            name: self.account_name.clone(),
                        },
                        &mut store.accounts,
                    ));
                }
                Some(acc) => self.account_id = acc.id,
            }
        }

        if self.amount_id.is_none() {
            // create Amount record
            self.amount_id = Some(Amount::save_to_store(
                Amount {
                    id: self.amount_id,
                    standard: self.amount_standard,
                    high: None,
                    low: None,
                },
                &mut store.amounts,
            ));
        }

        if self.income_id.is_none() {
            // try name lookup
            match Income::by_name(&self.income_name, &store.incomes) {
                None => {
                    // create Income record
                    let new_id = Income::save_to_store(
                        Income {
                            id: None,
                            active: true,
                            name: self.income_name.clone(),
                        },
                        &mut store.incomes,
                    );
                    self.income_id = Some(new_id);
                }
                Some(inc) => self.income_id = inc.id,
            }
        }

        self.payment_received_completed_at = match complete_at {
            None => Utc::now().naive_local(),
            Some(ndt) => ndt,
        };

        // create PaymentReceived record
        self.payment_received_id = Some(PaymentReceived::new_id(&store.payments_received));
        PaymentReceived::save_to_store(
            PaymentReceived {
                id: None,
                completed_at: self.payment_received_completed_at,
                account_id: self.account_id.unwrap(),
                amount_id: self.amount_id.unwrap(),
                income_id: self.income_id.unwrap(),
            },
            &mut store.payments_received,
        );

        // create new account balance record
        let prev_balance = Account::by_id(self.account_id.unwrap(), &mut store.accounts)
            .unwrap()
            .current_balance(&mut store.account_balances);
        self.prev_balance = Some(prev_balance);

        let ending_balance = prev_balance + self.amount_standard;
        self.ending_balance = Some(ending_balance);

        self.account_balance_id = Some(AccountBalance::save_to_store(
            AccountBalance {
                id: None,
                account_id: self.account_id.unwrap(),
                amount: ending_balance,
                reported_at: self.payment_received_completed_at,
            },
            &mut store.account_balances,
        ));

        Ok(())
    }
}

#[cfg(test)]
mod payment_composite_spec {
    use super::*;
    use crate::test::spec::Spec;
    use chrono::NaiveDate;

    fn payment_rec_comp() -> PaymentReceivedComposite {
        PaymentReceivedComposite {
            id: None,
            account_id: None,
            account_name: "piggybank".to_string(),
            account_balance_id: None,
            prev_balance: None,
            ending_balance: None,
            amount_id: None,
            amount_standard: Decimal::new(12345, 2),
            payment_received_id: None,
            payment_received_completed_at: NaiveDate::from_ymd_opt(2023, 2, 17)
                .unwrap()
                .and_hms_opt(13, 00, 00)
                .unwrap(),
            income_id: None,
            income_name: "cowboy".to_string(),
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn create_payment_received__retrieves_account_id_by_name_when_self_account_id_is_none_and_account_exists(
    ) {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut payment_rec_comp = payment_rec_comp();
        payment_rec_comp
            .create_payment_received(&mut store, None)
            .unwrap();
        assert_eq!(1, payment_rec_comp.account_id.unwrap());
    }

    #[test]
    #[allow(non_snake_case)]
    // TODO: eventually may want to change this behavior to raise an error saying account dne
    fn create_payment_received__creates_account_when_self_account_id_is_none_and_account_name_does_not_exist(
    ) {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut payment_rec_comp = payment_rec_comp();
        payment_rec_comp.account_name = "New Account".to_string();
        assert_eq!(2, store.accounts.len());
        payment_rec_comp
            .create_payment_received(&mut store, None)
            .unwrap();
        assert_eq!(3, store.accounts.len());
        assert_eq!(
            3,
            Account::by_name("New Account", &mut store.accounts)
                .unwrap()
                .id
                .unwrap()
        )
    }

    #[test]
    #[allow(non_snake_case)]
    fn create_payment_received__creates_amount_record_when_self_amount_id_is_none() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut payment_rec_comp = payment_rec_comp();
        assert_eq!(5, store.amounts.len());
        payment_rec_comp
            .create_payment_received(&mut store, None)
            .unwrap();
        assert_eq!(6, store.amounts.len());
    }

    #[test]
    #[allow(non_snake_case)]
    fn create_payment_received__retrieves_income_id_by_name_when_self_income_id_is_none_and_income_exists(
    ) {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut payment_rec_comp = payment_rec_comp();
        payment_rec_comp
            .create_payment_received(&mut store, None)
            .unwrap();
        assert_eq!(1, payment_rec_comp.income_id.unwrap());
    }

    #[test]
    #[allow(non_snake_case)]
    fn create_payment_received__creates_income_when_self_income_id_is_none_and_income_name_does_not_exist(
    ) {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut payment_rec_comp = payment_rec_comp();
        payment_rec_comp.income_name = "New Income".to_string();
        assert_eq!(2, store.incomes.len());
        payment_rec_comp
            .create_payment_received(&mut store, None)
            .unwrap();
        assert_eq!(3, store.incomes.len());
        assert_eq!(
            3,
            Income::by_name("New Income", &mut store.incomes)
                .unwrap()
                .id
                .unwrap()
        )
    }

    #[test]
    #[allow(non_snake_case)]
    // TODO: find out how to enact something like Ruby/Rspec's Timecop
    fn create_payment_received__sets_self_payment_completed_at_to_current_time_when_complete_at_is_none(
    ) {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut payment_rec_comp = payment_rec_comp();
        payment_rec_comp
            .create_payment_received(&mut store, None)
            .unwrap();
        assert_eq!(
            false,
            payment_rec_comp.payment_received_completed_at
                == NaiveDate::from_ymd_opt(2023, 2, 17)
                    .unwrap()
                    .and_hms_opt(13, 00, 00)
                    .unwrap()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn create_payment_received__sets_self_payment_completed_at_when_complete_at_is_passed_in() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut payment_rec_comp = payment_rec_comp();
        payment_rec_comp
            .create_payment_received(
                &mut store,
                Some(
                    NaiveDate::from_ymd_opt(3000, 4, 1)
                        .unwrap()
                        .and_hms_opt(13, 00, 00)
                        .unwrap(),
                ),
            )
            .unwrap();
        assert_eq!(
            NaiveDate::from_ymd_opt(3000, 4, 1)
                .unwrap()
                .and_hms_opt(13, 00, 00)
                .unwrap(),
            payment_rec_comp.payment_received_completed_at
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn create_payment_received__gets_current_balance_of_account() {
        // at this point in create_payment, account is guaranteed to exist
        // either existed before or was created by the method
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut payment_rec_comp = payment_rec_comp();
        payment_rec_comp
            .create_payment_received(&mut store, None)
            .unwrap();
        assert_eq!(Decimal::new(200, 0), payment_rec_comp.prev_balance.unwrap());
    }

    #[test]
    #[allow(non_snake_case)]
    fn create_payment_received__sets_self_prev_balance_to_0_when_new_account() {
        // at this point in create_payment, account is guaranteed to exist
        // either existed before or was created by the method
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut payment_rec_comp = payment_rec_comp();
        payment_rec_comp
            .create_payment_received(&mut store, None)
            .unwrap();
        assert_eq!(Decimal::new(200, 0), payment_rec_comp.prev_balance.unwrap());
    }

    #[test]
    #[allow(non_snake_case)]
    fn create_payment_received__creates_payment_received_record() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut payment_rec_comp = payment_rec_comp();
        assert_eq!(3, store.payments_received.len());
        payment_rec_comp
            .create_payment_received(&mut store, None)
            .unwrap();
        assert_eq!(4, store.payments.len());
        let new_payment = store.payments_received[&4].clone_record();
        assert_eq!(new_payment.account_id, payment_rec_comp.account_id.unwrap());
        assert_eq!(new_payment.income_id, payment_rec_comp.income_id.unwrap());
        assert_eq!(new_payment.amount_id, payment_rec_comp.amount_id.unwrap());
        assert_eq!(
            new_payment.completed_at,
            payment_rec_comp.payment_received_completed_at
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn create_payment_received__creates_account_balance_record() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let mut payment_rec_comp = payment_rec_comp();
        assert_eq!(4, store.account_balances.len());
        payment_rec_comp
            .create_payment_received(&mut store, None)
            .unwrap();
        assert_eq!(5, store.account_balances.len());
        let new_acc_bal = store.account_balances[&5].clone_record();
        assert_eq!(new_acc_bal.account_id, payment_rec_comp.account_id.unwrap());
        assert_eq!(new_acc_bal.amount, payment_rec_comp.ending_balance.unwrap());
        assert_eq!(
            new_acc_bal.reported_at,
            payment_rec_comp.payment_received_completed_at
        )
    }
}
