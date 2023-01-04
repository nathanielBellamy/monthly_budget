pub struct Expense<'a> {
    pub active: bool,
    pub amount: usize,
    pub name: &'a str,
    pub recurring: bool,
}
