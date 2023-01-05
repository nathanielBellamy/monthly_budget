use crate::lib::money::tag::Tag;

pub struct Expense<'a> {
    pub active: bool,
    pub amount: usize,
    pub name: &'a str,
    pub recurrence_id: Option<usize>,
    pub tags: Option<Vec<Tag<'a>>>,
}
