use crate::store::store::Store;
pub struct Initializer;

impl<'a> Initializer {
    pub fn init(store: &mut Store) -> &mut Store {
        store.init(Some("src/spec/data/")).unwrap()
    }
}
