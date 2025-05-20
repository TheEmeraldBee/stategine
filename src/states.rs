use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
};

pub type States = HashMap<TypeId, RefCell<Box<dyn Any>>>;

pub trait MultiInsertStates {
    fn insert(self, states: &mut States);
}

macro_rules! impl_multi_insert_states {
    ($($item:ident $num:tt)*) => {
        impl<$($item: Any + 'static),*> MultiInsertStates for ($($item,)*) {
            fn insert(self, states: &mut States) {
                $(states.insert(TypeId::of::<$item>(), RefCell::new(Box::new(self.$num)));)*
            }
        }
    }
}

impl_multi_insert_states! { A 0 }
impl_multi_insert_states! { A 0 B 1 }
impl_multi_insert_states! { A 0 B 1 C 2 }
impl_multi_insert_states! { A 0 B 1 C 2 D 3 }
impl_multi_insert_states! { A 0 B 1 C 2 D 3 E 4 }
impl_multi_insert_states! { A 0 B 1 C 2 D 3 E 4 F 5 }
impl_multi_insert_states! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 }
impl_multi_insert_states! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 }
impl_multi_insert_states! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 }
impl_multi_insert_states! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9}
