use std::any::type_name;
use std::marker::PhantomData;

use std::{
    any::{Any, TypeId},
    cell::Ref,
    ops::Deref,
};

use crate::EntityStateStorage;

use super::SystemParam;

pub struct Res<'a, T: 'static> {
    value: Ref<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: 'static> Deref for Res<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value.downcast_ref().unwrap()
    }
}

impl<'a, T: 'static> SystemParam for Res<'a, T> {
    type Item<'new> = Res<'new, T>;
    fn retrieve(resources: &EntityStateStorage) -> Self::Item<'_> {
        Res {
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
                .borrow(),
            _marker: PhantomData,
        }
    }
}
