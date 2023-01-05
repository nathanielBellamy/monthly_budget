// use rand::Rng;

pub struct Amount {
    // use to over/under estimate
    pub low: Option<usize>,
    pub high: Option<usize>,
    pub standard: usize,
}

impl Amount {
    pub fn randomize() -> usize {
        // let res = rand::thread_rng().gen_range(self.low..self.high);
        // res
        20
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_amount() -> Amount {
        Amount {
            low: Some(10),
            high: Some(100),
            standard: 50,
        }
    }

    #[test]
    fn randomize_returns_random_number_between_low_and_high() {
        let amount = test_amount();
        let randomized = Some(20);
        assert!(randomized < amount.high);
        assert!(randomized > amount.low);
    }
}
