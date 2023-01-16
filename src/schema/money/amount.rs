use crate::spec::spec::Spec;
use crate::store::store::Store;
use crate::traits::csv_store::CsvStore;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Amount {
    pub id: usize,
    pub standard: f64,
    // use to over/under estimate
    pub low: Option<f64>,
    pub high: Option<f64>,
}

impl CsvStore for Amount {}

impl Amount {
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

    #[test]
    #[allow(non_snake_case)]
    fn randomize__returns_random_number_between_low_and_high() {
        let mut store = Store::new();
        Spec::init(&mut store);

        let amount = &store.amounts[2]; // first amount in spec data with high, low
        let randomized = amount.randomize();
        assert!(randomized < amount.high.unwrap());
        assert!(randomized > amount.low.unwrap());
    }
}
