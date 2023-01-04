pub struct Amount {
    // use to over/under estimate
    pub low: usize,
    pub high: usize,
    pub average: usize,
}

impl Amount {
    pub fn randomize() -> usize {
        // random number between low and high
        0
    }
}
