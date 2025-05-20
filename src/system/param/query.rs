use std::marker::PhantomData;

use std::{
    any::{Any, TypeId},
    cell::Ref,
};

use crate::EntityStateStorage;

use super::SystemParam;

pub struct Query<'a, T: 'static> {
    value: Vec<Ref<'a, Box<dyn Any>>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: 'static> Query<'a, T> {
    pub fn get<'b>(&'b self, idx: usize) -> Option<&'b T> {
        self.value.get(idx).map(|x| x.downcast_ref().unwrap())
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

impl<'a, 'b, T: 'static> Iterator for QueryIter<'a, 'b, T> {
    type Item = &'b T;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.query.get(self.item);
        self.item += 1;
        res
    }
}

impl<'a, T: 'static> SystemParam for Query<'_, T> {
    type Item<'new> = Query<'new, T>;

    fn retrieve(resources: &EntityStateStorage) -> Self::Item<'_> {
        Query {
            value: resources
                .entities
                .iter()
                .filter(|x| x.0 == TypeId::of::<T>())
                .map(|x| x.1.borrow())
                .collect::<Vec<_>>(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T: 'static> IntoIterator for &'a Query<'a, T> {
    type Item = &'a T;
    type IntoIter = QueryIter<'a, 'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        QueryIter {
            query: self,
            item: 0,
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
        let res = query.iter().map(|x| *x).collect::<Vec<_>>();
        assert_eq!(res, vec![12, 5, 12]);
    }
}
