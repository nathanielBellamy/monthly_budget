use crate::store::store::Store;
pub struct Spec;

impl<'a> Spec {
    #[allow(unused)] // Used in test mods
    pub fn init(store: &mut Store) -> &mut Store {
        store.init(Some("src/spec/data/")).unwrap()
    }
}
