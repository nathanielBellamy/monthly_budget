use crate::biblio::calendar;
use crate::store::store::Store;

mod biblio;
mod store;
mod traits;

fn main() {
    let store = Store::new().init();
}
