use crate::store::store::Store;
pub struct Spec;

impl<'a> Spec {
    pub fn init(store: &mut Store) -> &mut Store {
        store.init(Some("src/spec/data/")).unwrap()
    }
}
