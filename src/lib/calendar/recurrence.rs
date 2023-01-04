pub struct Recurrence {
    pub id: u16,
}

pub enum CycleEvery {
    Day(u8),
    Week(u8),
    Month(u8),
    Year(u8),
}
