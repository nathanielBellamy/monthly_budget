use crate::schema::account::Account;
use crate::storage::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use chrono::NaiveDateTime;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountSummary {
    pub id: Option<usize>,
    pub name: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub balance: Decimal,
    pub reported_at: NaiveDateTime,
}

impl AccountSummary {
    pub fn clone(&self) -> AccountSummary {
        AccountSummary {
            name: self.name.clone(),
            ..*self
        }
    }

    pub fn by_id(id: usize, store: &mut Store) -> AccountSummaryStore {
        let mut account_summary_store = AccountSummaryStore::new();
        let account = Account::by_id(id, &mut store.accounts).unwrap();
        for (ab_id, account_balance) in store.account_balances.iter() {
            if account_balance.account_id == id {
                AccountSummary::save_to_store(
                    AccountSummary {
                        id: Some(*ab_id),
                        name: account.name.clone(),
                        balance: account_balance.amount,
                        reported_at: account_balance.reported_at,
                    },
                    &mut account_summary_store,
                );
            }
        }

        account_summary_store
    }
}

pub type AccountSummaryStore = BTreeMap<usize, AccountSummary>;

impl CsvRecord<AccountSummary> for AccountSummary {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
        self.id = Some(new_id);
        self.id
    }

    fn clone_record(&self) -> AccountSummary {
        self.clone()
    }
}

impl CsvStore<AccountSummary> for AccountSummary {}

#[cfg(test)]
mod account_summary_spec {
    use super::*;
    use crate::test::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn by_id__gathers_all_account_balances_for_account_by_id() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let account_summary = AccountSummary::by_id(1, &mut store);
        assert_eq!(2, account_summary.len());
        assert_eq!("piggybank".to_string(), account_summary[&1].name);
        assert_eq!("piggybank".to_string(), account_summary[&2].name);
    }
}
