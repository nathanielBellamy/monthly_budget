pub struct Income<'a> {
    pub active: bool,
    pub amount: usize,
    pub name: &'a str,
    pub recurring: bool,
    /*
      start_month
      end_month
      tags - add tags to be able to run different scenarios
    */
}
