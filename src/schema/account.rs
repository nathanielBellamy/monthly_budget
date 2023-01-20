use crate::schema::account_balance::{AccountBalance, AccountBalanceStore};
use crate::store::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: Option<usize>,
    pub name: String,
}

impl CsvRecord<Account> for Account {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
      self.id = Some(new_id);
      self.id
    }

    fn clone_record(&self) -> Account {
        Account {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

impl CsvStore<Account> for Account {}

pub type AccountStore = HashMap<usize, Account>;

impl Account {
    // TODO: maybe avoid clone, return id
    pub fn by_name<'a, 'b: 'a>(name: &'a str, store: &'b AccountStore) -> Option<Account> {
        let mut account: Option<Account> = None;
        for (_id, acc) in store.iter() {
            if acc.name.to_owned() == name {
                account = Some(acc.clone_record());
                break;
            }
        }
        account
    }

    pub fn current_balance(&self, store: &AccountBalanceStore) -> Option<f64> {
        let mut balance: Option<AccountBalance> = None;
        for (id, bal) in store.iter() {
            // most recently pushed balance
            if *id == self.id.unwrap() {
                balance = Some(*bal);
                break;
            }
        }
        match balance {
            None => None,
            Some(bal) => Some(bal.amount),
        }
    }

    pub fn find_or_create<'a, 'b: 'a>(
        id: Option<usize>,
        name: &'a str,
        store: &'b mut Store,
    ) -> Option<Account> {
        match id {
            None => Account::by_name(&name[..], &mut store.accounts),
            Some(id) => Account::by_id(id, &mut store.accounts), // failure here indicates id mismatch data
        }
    }
}

#[cfg(test)]
mod account_spec {
    use super::*;
    use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn by_id__returns_record_from_store() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let account = Account::by_id(1, &mut store.accounts).unwrap();
        assert_eq!("piggybank", account.name)
    }

    #[test]
    #[allow(non_snake_case)]
    fn by_name__returns_record_from_store() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let account = Account::by_name("piggybank", &mut store.accounts).unwrap();
        assert_eq!(1, account.id.unwrap())
    }
}
