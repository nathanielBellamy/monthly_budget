pub struct Recurrence {
    pub id: usize,
    pub start_date_id: usize,
    pub end_date_id: usize,
    pub cycle_length: CycleLength,
}

pub enum CycleLength {
    Day(u8),
    Week(u8),
    Month(u8),
    Year(u8),
}
