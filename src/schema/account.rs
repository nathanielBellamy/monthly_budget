use crate::schema::account_balance::{AccountBalance, AccountBalanceStore};
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::btree_map::Entry;
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
            if acc.name == name {
                account = Some(acc.clone_record());
                break;
            }
        }
        account
    }

    pub fn account_balance_ids(&self, store: &mut AccountBalanceStore) -> Vec<usize> {
        let mut balance_ids: Vec<usize> = vec![];
        for (id, acc_bal) in store.iter() {
            if acc_bal.account_id == self.id.unwrap() {
                balance_ids.push(*id)
            }
        }

        balance_ids
    }

    // last_saved_balance
    pub fn current_balance(&self, store: &mut AccountBalanceStore) -> Decimal {
        let mut curr_balance: Option<AccountBalance> = None;
        for id in self.account_balance_ids(store).iter() {
            if let Entry::Occupied(acc_bal) = store.entry(*id) {
                // entry exists
                match curr_balance {
                    None => {
                        // set first
                        curr_balance = Some(acc_bal.get().clone_record());
                    }
                    Some(last_acc_bal_so_far) => {
                        if acc_bal.get().reported_at > last_acc_bal_so_far.reported_at {
                            curr_balance = Some(acc_bal.get().clone_record())
                        }
                    }
                }
            }
        }

        match curr_balance {
            None => Decimal::new(00, 1),
            Some(bal) => bal.amount,
        }
    }
}

#[cfg(test)]
mod account_spec {
    use super::*;
    use crate::storage::store::Store;
    use crate::test::spec::Spec;

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

    #[test]
    #[allow(non_snake_case)]
    fn current_balance__returns_amount_of_account_balance_with_most_recent_reported_at() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let account = Account::by_name("piggybank", &mut store.accounts).unwrap();
        assert_eq!(
            Decimal::new(2000, 1),
            account.current_balance(&mut store.account_balances)
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn account_balance_ids__returns_vec_of_ids() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let account = Account::by_name("piggybank", &mut store.accounts).unwrap();
        let account_balance_ids = account.account_balance_ids(&mut store.account_balances);
        assert_eq!(vec![1, 2], account_balance_ids);

        for id in account_balance_ids.iter() {
            let balance = AccountBalance::by_id(*id, &mut store.account_balances).unwrap();
            assert_eq!(balance.account_id, account.id.unwrap());
        }
    }
}
