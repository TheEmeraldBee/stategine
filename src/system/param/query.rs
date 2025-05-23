use std::marker::PhantomData;

use std::{
    any::{Any, TypeId},
    cell::Ref,
};

use uuid::Uuid;

use crate::EntityStateStorage;

use super::SystemParam;

pub struct Query<'a, T: 'static> {
    value: Vec<(uuid::Uuid, Ref<'a, Box<dyn Any>>)>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: 'static> Query<'a, T> {
    pub fn get(&self, idx: usize) -> Option<(uuid::Uuid, &T)> {
        self.value
            .get(idx)
            .map(|x| (x.0, x.1.downcast_ref().unwrap()))
    }

    pub fn iter<'b>(&'b self) -> QueryIter<'a, 'b, T> {
        QueryIter {
            query: self,
            item: 0,
        }
    }
}

pub struct QueryIter<'a, 'b, T: 'static> {
    query: &'b Query<'a, T>,
    item: usize,
}

impl<'b, T: 'static> Iterator for QueryIter<'_, 'b, T> {
    type Item = (Uuid, &'b T);
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.query.get(self.item);
        self.item += 1;
        res
    }
}

impl<T: 'static> SystemParam for Query<'_, T> {
    type Item<'new> = Query<'new, T>;

    fn retrieve(resources: &EntityStateStorage) -> Self::Item<'_> {
        Query {
            value: resources
                .entities
                .iter()
                .filter(|x| x.0 == TypeId::of::<T>())
                .map(|x| (x.1, x.2.borrow()))
                .collect::<Vec<_>>(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::EntityStateStorage;

    use super::{Query, SystemParam};

    #[test]
    fn test_query() {
        let mut storage = EntityStateStorage::default();
        storage.entity(12).entity(5).entity(12);

        let query = Query::<'_, i32>::retrieve(&storage);
        let res = query.iter().map(|x| *x.1).collect::<Vec<_>>();
        assert_eq!(res, vec![12, 5, 12]);
    }
}
