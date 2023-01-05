use rand::Rng;

pub struct Amount {
    // use to over/under estimate
    pub low: Option<usize>,
    pub high: Option<usize>,
    pub standard: usize,
}

impl Amount {
    pub fn randomize(&self) -> usize {
        let mut _low: usize = 0;
        let mut _high: usize = 1;
        match self.low {
            Some(low) => _low = low,
            _ => (),
        }
        match self.high {
            Some(high) => _high = high,
            _ => (),
        }
        let res = rand::thread_rng().gen_range(_low.._high);
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
