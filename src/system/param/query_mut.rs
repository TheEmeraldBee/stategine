use std::borrow::BorrowMut;
use std::marker::PhantomData;
use std::{
    any::{Any, TypeId},
    cell::RefMut,
};

use uuid::Uuid;

use crate::EntityStateStorage;

use super::SystemParam;

pub struct QueryMut<'a, T: 'static> {
    value: Vec<(Uuid, RefMut<'a, Box<dyn Any>>)>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: 'static> QueryMut<'a, T> {
    pub fn get_mut(&mut self, idx: usize) -> Option<(Uuid, &mut T)> {
        self.value
            .get_mut(idx)
            .map(|x| (x.0, x.1.downcast_mut().unwrap()))
    }

    pub fn iter_mut<'b>(&'b mut self) -> QueryMutIter<'a, 'b, T> {
        QueryMutIter {
            query: self,
            item: 0,
        }
    }
}

pub struct QueryMutIter<'a, 'b, T: 'static> {
    query: &'b mut QueryMut<'a, T>,
    item: usize,
}

impl<'b, T: 'static> Iterator for QueryMutIter<'_, 'b, T> {
    type Item = (Uuid, &'b mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.item < self.query.value.len() {
            let idx = self.item;
            self.item += 1;

            // SAFETY index is valid, bounds have been checked
            let ref_mut: (Uuid, &'b mut Box<dyn Any>) = unsafe {
                let ptr = self.query.value.as_mut_ptr().add(idx);

                ((*ptr).0, (*ptr).1.borrow_mut())
            };

            let value: &'b mut T = ref_mut
                .1
                .downcast_mut::<T>()
                .expect("Failed to downcast Box<dyn Any> to the expected type");

            Some((ref_mut.0, value))
        } else {
            None
        }
    }
}

impl<T: 'static> SystemParam for QueryMut<'_, T> {
    type Item<'new> = QueryMut<'new, T>;

    fn retrieve(resources: &EntityStateStorage) -> Self::Item<'_> {
        QueryMut {
            value: resources
                .entities
                .iter()
                .filter(|x| x.0 == TypeId::of::<T>())
                .map(|x| (x.1, x.2.borrow_mut()))
                .collect::<Vec<_>>(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{EntityStateStorage, system::param::query::Query};

    use super::{QueryMut, SystemParam};

    #[test]
    fn test_query_mut() {
        let mut storage = EntityStateStorage::default();
        storage.entities([12, 5, 12]);

        let mut query = QueryMut::<i32>::retrieve(&storage);
        query.iter_mut().for_each(|x| *x.1 += 5);
        drop(query);

        let query = Query::<i32>::retrieve(&storage);
        let res = query.iter().map(|x| *x.1).collect::<Vec<_>>();

        assert_eq!(res, vec![17, 10, 17]);
    }
}
