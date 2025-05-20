use crate::EntityStateStorage;

pub mod query;
pub mod query_mut;
pub mod res;
pub mod res_mut;

pub trait SystemParam {
    type Item<'new>;
    fn retrieve(resources: &EntityStateStorage) -> Self::Item<'_>;
}
