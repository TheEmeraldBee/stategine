use crate::system::{System, SystemResult, into_system::IntoSystem, param::SystemParam};

use crate::EntityStateStorage;

use std::marker::PhantomData;

// A widget that comes from specifically a function.
pub struct FunctionSystem<T, Input, F> {
    f: F,
    marker: PhantomData<fn() -> (T, Input)>,
}

macro_rules! impl_for_func {
    ($($item:ident)*) => {
        impl<T, Func, $($item),*> System<T> for FunctionSystem<T, ($($item,)*), Func>
        where
                for<'a, 'b> &'a mut Func:
                    FnMut( $($item),* ) -> SystemResult<T> +
                    FnMut( $(<$item as SystemParam>::Item<'b>),* ) -> SystemResult<T>,
            $($item: SystemParam + 'static),*
        {
            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn call(&mut self, storage: &mut EntityStateStorage) -> SystemResult<T> {
                #[allow(clippy::too_many_arguments)]
                fn call_inner<T, $($item),*>(
                    mut f: impl FnMut($($item),*) -> SystemResult<T>,
                    $($item: $item,)*
                ) -> SystemResult<T> {
                    f($($item),*)
                }

                $(
                    let $item = $item::retrieve(storage);
                )*

                call_inner(&mut self.f, $($item),*)
            }
        }
    };
}

impl_for_func! {}
impl_for_func! { A }
impl_for_func! { A B }
impl_for_func! { A B C }
impl_for_func! { A B C D }
impl_for_func! { A B C D E }
impl_for_func! { A B C D E F }
impl_for_func! { A B C D E F G }
impl_for_func! { A B C D E F G H }
impl_for_func! { A B C D E F G H I }
impl_for_func! { A B C D E F G H I J }
impl_for_func! { A B C D E F G H I J K }
impl_for_func! { A B C D E F G H I J K L }

macro_rules! impl_into_system {
    ($($item:ident)*) => {
        impl<T, Func, $($item),*> IntoSystem<T, ($($item,)*), ()> for Func
                where
                for<'a, 'b> &'a mut Func:
                    FnMut( $($item),* ) -> SystemResult<T> +
                    FnMut( $(<$item as SystemParam>::Item<'b>),* ) -> SystemResult<T>,
            $($item: SystemParam + 'static),*

        {
            type System = FunctionSystem<T, ($($item,)*), Self>;
            #[inline]
            #[allow(non_snake_case, unused_variables)]
            fn into_widget(self) -> Self::System {
                FunctionSystem {
                    f: self,
                    marker: Default::default(),
                }
            }
        }
    };
}

impl_into_system! {}
impl_into_system! { A }
impl_into_system! { A B }
impl_into_system! { A B C }
impl_into_system! { A B C D }
impl_into_system! { A B C D E }
impl_into_system! { A B C D E F }
impl_into_system! { A B C D E F G }
impl_into_system! { A B C D E F G H }
impl_into_system! { A B C D E F G H I }
impl_into_system! { A B C D E F G H I J }
impl_into_system! { A B C D E F G H I J K }
impl_into_system! { A B C D E F G H I J K L }
