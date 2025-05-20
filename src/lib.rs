use std::{
    any::{Any, TypeId},
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use states::{MultiInsertStates, States};
use system::{
    System,
    into_system::IntoSystem,
    into_system_set::IntoSystemSet,
    param::{SystemParam, query::Query, query_mut::QueryMut, res::Res, res_mut::ResMut},
};

pub mod entity;
pub mod prelude;
pub mod states;
pub mod system;

pub struct Engine<T> {
    systems: Vec<Box<dyn System<T>>>,
    pub(crate) storage: EntityStateStorage,
}

impl<T> Default for Engine<T> {
    fn default() -> Self {
        Self {
            systems: vec![],
            storage: EntityStateStorage::default(),
        }
    }
}

impl<T> Deref for Engine<T> {
    type Target = EntityStateStorage;
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl<T> DerefMut for Engine<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}

impl<T> Engine<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn oneshot_system(&mut self, mut system: impl System<T>) -> T {
        system.call(&mut self.storage)
    }

    pub fn non_auto_system<S: System<T> + 'static>(&mut self, system: S) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn system<I, A, S: System<T> + 'static, M: IntoSystem<T, I, A, System = S> + 'static>(
        &mut self,
        system: M,
    ) -> &mut Self {
        self.non_auto_system(system.into_system())
    }

    pub fn systems<I, A>(&mut self, systems: impl IntoSystemSet<T, I, A>) -> &mut Self {
        for system in systems.into_widget_set() {
            self.systems.push(system);
        }
        self
    }

    pub fn update(&mut self) {
        for system in &mut self.systems {
            system.call(&mut self.storage);
        }
    }
}

#[derive(Default)]
pub struct EntityStateStorage {
    pub(crate) entities: Vec<(TypeId, RefCell<Box<dyn Any>>)>,
    pub(crate) states: States,
}

impl EntityStateStorage {
    pub fn entity<T: Any + 'static>(&mut self, entity: T) -> &mut Self {
        self.entities
            .push((TypeId::of::<T>(), RefCell::new(Box::new(entity))));
        self
    }

    pub fn state<T: Any + 'static>(&mut self, state: T) -> &mut Self {
        self.states
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(state)));
        self
    }
    pub fn states(&mut self, states: impl MultiInsertStates) -> &mut Self {
        states.insert(&mut self.states);
        self
    }

    pub fn get_state<T>(&self) -> Res<T> {
        Res::retrieve(self)
    }
    pub fn get_state_mut<T>(&self) -> ResMut<T> {
        ResMut::retrieve(self)
    }

    pub fn query<T>(&self) -> Query<T> {
        Query::retrieve(self)
    }
    pub fn query_mut<T>(&self) -> QueryMut<T> {
        QueryMut::retrieve(self)
    }
}
