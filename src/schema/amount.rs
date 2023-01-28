use crate::traits::csv_record::CsvRecord;
use crate::traits::csv_store::CsvStore;
use rand::Rng;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Amount {
    pub id: Option<usize>,
    #[serde(with = "rust_decimal::serde::float")]
    pub standard: Decimal,
    // use to over/under estimate
    #[serde(with = "rust_decimal::serde::float_option")]
    pub low: Option<Decimal>,
    #[serde(with = "rust_decimal::serde::float_option")]
    pub high: Option<Decimal>,
}

impl CsvRecord<Amount> for Amount {
    fn id(&self) -> Option<usize> {
        self.id
    }

    fn set_id(&mut self, new_id: usize) -> Option<usize> {
        self.id = Some(new_id);
        self.id
    }

    fn clone_record(&self) -> Amount {
        *self
    }
}
impl CsvStore<Amount> for Amount {}

pub type AmountStore = BTreeMap<usize, Amount>;

impl Amount {
    #[allow(unused)]
    pub fn randomize(&self) -> Decimal {
        let mut low = Decimal::new(00, 1);
        if let Some(num) = self.low {
            low = num
        }
        let high: Decimal = match self.high {
            Some(num) => num,
            _ => self.standard * Decimal::new(30, 1), // TODO: 3 is a magic number here
                                                        //   tune logic for useful randomization
        };
        rand::thread_rng().gen_range(low..high)
    }
}

#[cfg(test)]
mod amount_spec {
    use super::*;
    use crate::test::spec::Spec;
    use crate::storage::store::Store;

    #[test]
    #[allow(non_snake_case)]
    fn by_id__returns_record_from_store() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let amount = Amount::by_id(1, &mut store.amounts).unwrap();
        assert_eq!(Decimal::new(310000, 2), amount.standard);
    }

    #[test]
    #[allow(non_snake_case)]
    fn randomize__returns_random_number_between_low_and_high() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let amount = Amount::by_id(3, &mut store.amounts).unwrap(); // first amount in spec data with high, low
        let randomized = amount.randomize();
        assert!(randomized < amount.high.unwrap());
        assert!(randomized > amount.low.unwrap());
    }
}
