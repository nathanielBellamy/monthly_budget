use crate::storage::store::Store;

mod end_to_end;

pub struct Spec;

impl Spec {
    #[allow(unused)] // Used in test mods
    pub fn init(store: &mut Store) -> &mut Store {
        store.init(Some("src/test/data/".to_string())).unwrap()
    }
}
