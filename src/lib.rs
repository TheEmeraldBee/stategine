use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use prelude::Commands;
use states::{MultiInsertStates, States};
use system::{
    System,
    into_system_set::IntoSystemSet,
    param::{SystemParam, query::Query, query_mut::QueryMut, res::Res, res_mut::ResMut},
};
use uuid::Uuid;

pub mod prelude;
pub mod states;
pub mod system;

pub mod commands;

#[derive(Default)]
pub struct Engine {
    systems: Vec<Box<dyn System<()>>>,
    pub(crate) storage: EntityStateStorage,
}

impl Deref for Engine {
    type Target = EntityStateStorage;
    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl DerefMut for Engine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn oneshot_system<T>(&mut self, mut system: impl System<T>) -> T {
        system.call(&mut self.storage)
    }

    pub fn system<S: System<()> + 'static>(&mut self, system: S) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn systems<I, A>(&mut self, systems: impl IntoSystemSet<(), I, A>) -> &mut Self {
        for system in systems.into_widget_set() {
            self.systems.push(system);
        }
        self
    }

    pub fn update(&mut self) {
        for system in &mut self.systems {
            system.call(&mut self.storage);
        }

        // Apply all commands
        let commands = self.take_state::<Commands>();
        commands.apply_all(self);
        self.state(Commands::default());
    }
}

pub struct EntityStateStorage {
    pub(crate) entities: Vec<(TypeId, Uuid, RefCell<Box<dyn Any>>)>,
    pub(crate) states: States,
}

impl Default for EntityStateStorage {
    fn default() -> Self {
        let mut res = Self {
            entities: Vec::new(),
            states: HashMap::new(),
        };
        res.states((Commands::default(),));
        res
    }
}

impl EntityStateStorage {
    pub fn entity<T: Any + 'static>(&mut self, entity: T) -> &mut Self {
        self.entities.push((
            TypeId::of::<T>(),
            Uuid::new_v4(),
            RefCell::new(Box::new(entity)),
        ));
        self
    }

    pub fn remove_entity(&mut self, uuid: Uuid) -> &mut Self {
        self.entities.retain(|x| x.1 != uuid);
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

    pub fn take_state<T: 'static>(&mut self) -> T {
        *self
            .states
            .remove(&TypeId::of::<T>())
            .expect("State should exist")
            .into_inner()
            .downcast()
            .unwrap()
    }

    pub fn query<T>(&self) -> Query<T> {
        Query::retrieve(self)
    }
    pub fn query_mut<T>(&self) -> QueryMut<T> {
        QueryMut::retrieve(self)
    }
}
