use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: usize,
    pub name: &'static str,
}

impl Account {
    pub fn release_funds(&self, from_acc: &Account) -> usize {
        //
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_funds_subtracts_ammount_from_acc() {}
}