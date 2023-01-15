use crate::traits::csv_store::CsvStore;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    pub id: usize,
    pub standard: f64,
    // use to over/under estimate
    pub low: Option<f64>,
    pub high: Option<f64>,
}

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

impl CsvStore for Amount {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randomize_returns_random_number_between_low_and_high() {
        let amount = test_amount();
        let randomized = amount.randomize();
        assert!(randomized < amount.high.unwrap());
        assert!(randomized > amount.low.unwrap());
    }

    fn test_amount() -> Amount {
        Amount {
            id: 1,
            standard: 50.0,
            low: Some(10.0),
            high: Some(100.0),
        }
    }
}
