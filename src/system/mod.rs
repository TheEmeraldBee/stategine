use crate::EntityStateStorage;

pub mod function_system;
pub mod into_system;
pub mod into_system_set;
pub mod param;

pub mod conditional_system_set;

pub trait System<T> {
    fn call(&mut self, storage: &mut EntityStateStorage) -> T;
}
