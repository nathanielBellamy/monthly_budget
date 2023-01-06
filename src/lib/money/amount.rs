use rand::Rng;

pub struct Amount {
    // use to over/under estimate
    pub low: Option<usize>,
    pub high: Option<usize>,
    pub standard: usize,
}

impl Amount {
    pub fn randomize(&self) -> usize {
        let mut low: usize = 0;
        let high: usize;
        match self.low {
            Some(num) => low = num,
            _ => (),
        }
        match self.high {
            Some(num) => high = num,
            _ => high = self.standard * 3, // TODO: 3 is a magic number here
                                           //   tune logic for useful randomization
        }
        let res = rand::thread_rng().gen_range(low..high);
        res
    }
}

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
            low: Some(10),
            high: Some(100),
            standard: 50,
        }
    }
}
