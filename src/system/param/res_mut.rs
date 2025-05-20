use std::marker::PhantomData;

use std::{
    any::{Any, TypeId, type_name},
    cell::RefMut,
    ops::{Deref, DerefMut},
};

use crate::EntityStateStorage;

use super::SystemParam;

pub struct ResMut<'a, T: 'static> {
    value: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a mut T>,
}

impl<T: 'static> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.downcast_ref().unwrap()
    }
}

impl<T: 'static> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.downcast_mut().unwrap()
    }
}

impl<'a, T: 'static> SystemParam for ResMut<'a, T> {
    type Item<'new> = ResMut<'new, T>;

    fn retrieve(resources: &EntityStateStorage) -> Self::Item<'_> {
        ResMut {
            value: resources
                .states
                .get(&TypeId::of::<T>())
                .unwrap_or_else(|| {
                    panic!(
                        "Resource: `{}` with id `{:?}` Not Found",
                        type_name::<T>(),
                        TypeId::of::<T>()
                    )
                })
                .borrow_mut(),
            _marker: PhantomData,
        }
    }
}
