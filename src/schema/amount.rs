use crate::store::store::Store;
use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Amount {
    pub id: usize,
    pub standard: f64,
    // use to over/under estimate
    pub low: Option<f64>,
    pub high: Option<f64>,
}

impl CsvRecord<Amount> for Amount {
    fn id(&self) -> usize {
        self.id
    }

    fn clone_record(&self) -> Amount {
        self.clone()
    }
}
impl CsvStore for Amount {}

pub type AmountStore = HashMap<usize, Amount>;

impl Amount {
    pub fn by_id(id: usize, store: &Store) -> Option<&Amount> {
        let mut amount: Option<&Amount> = None;
        for (amt_id, amt) in store.amounts.iter() {
            if *amt_id == id {
                amount = Some(amt);
                break;
            }
        }
        amount
    }

    pub fn randomize(&self) -> f64 {
        let mut low: f64 = 0.0;
        let high: f64;
        if let Some(num) = self.low {
            low = num
        }
        match self.high {
            Some(num) => high = num,
            _ => high = self.standard * 3.0, // TODO: 3 is a magic number here
                                             //   tune logic for useful randomization
        }
        rand::thread_rng().gen_range(low..high)
    }
}

#[cfg(test)]
mod amount_spec {
    use super::*;
    use crate::spec::spec::Spec;

    #[test]
    #[allow(non_snake_case)]
    fn by_id__returns_record_from_store() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let amount = Amount::by_id(1, &mut store).unwrap();
        assert_eq!(3100.00, amount.standard);
    }

    #[test]
    #[allow(non_snake_case)]
    fn randomize__returns_random_number_between_low_and_high() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let amount = Amount::by_id(3, &mut store).unwrap(); // first amount in spec data with high, low
        let randomized = amount.randomize();
        assert!(randomized < amount.high.unwrap());
        assert!(randomized > amount.low.unwrap());
    }
}
