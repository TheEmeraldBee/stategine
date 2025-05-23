use std::any::Any;

use uuid::Uuid;

use crate::Engine;

pub trait Command {
    fn apply(self: Box<Self>, engine: &mut Engine);
}

pub enum EntityCommand<T: 'static> {
    Add(T),
    Remove(Uuid),
}

impl<T> Command for EntityCommand<T> {
    fn apply(self: Box<Self>, engine: &mut Engine) {
        match *self {
            Self::Add(e) => engine.entity(e),
            Self::Remove(d) => engine.remove_entity(d),
        };
    }
}

#[derive(Default)]
pub struct Commands {
    events: Vec<Box<dyn Command>>,
}

impl Commands {
    pub fn add(&mut self, event: impl Command + 'static) {
        self.events.push(Box::new(event))
    }

    pub fn entity<T: Any + 'static>(&mut self, e: T) {
        self.add(EntityCommand::Add(e));
    }

    pub fn remove_entity(&mut self, uuid: Uuid) {
        self.add(EntityCommand::Remove::<()>(uuid))
    }

    pub fn apply_all(self, engine: &mut Engine) {
        self.events.into_iter().for_each(|x| x.apply(engine));
    }
}
