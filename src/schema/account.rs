use crate::schema::account_balance::{AccountBalance, AccountBalanceStore};
use crate::store::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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

pub type AccountStore = BTreeMap<usize, Account>;

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

    // last_saved_balance
    pub fn current_balance(&self, store: &mut AccountBalanceStore) -> f64 {
        let mut curr_balance: Option<AccountBalance> = None;
        for (_id, acc_bal) in store.iter() {
            match curr_balance {
              None => curr_balance = Some(*acc_bal), // set first
              Some(last_acc_bal_so_far) => {
                if acc_bal.reported_at > last_acc_bal_so_far.reported_at {
                  curr_balance = Some(*acc_bal)
                }
              }
            }
        }

        match curr_balance {
          None => 0.0,
          Some(bal) => bal.amount
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
