use crate::EntityStateStorage;

use super::{System, into_system::IntoSystem, into_system_set::IntoSystemSet};

pub struct ConditionalSystemSet<S: System<bool>> {
    condition: S,
    widgets: Vec<Box<dyn System<()>>>,
}

impl<CS: System<bool>> ConditionalSystemSet<CS> {
    pub fn new<
        CI,
        CA,
        C: IntoSystem<bool, CI, CA, System = CS>,
        I,
        A,
        S: IntoSystemSet<(), I, A>,
    >(
        condition: C,
        systems: S,
    ) -> Self {
        Self {
            condition: condition.into_system(),
            widgets: systems.into_widget_set(),
        }
    }
}

impl<S: System<bool>> System<()> for ConditionalSystemSet<S> {
    fn call(&mut self, storage: &mut EntityStateStorage) {
        if self.condition.call(storage) {
            for widget in &mut self.widgets {
                widget.call(storage);
            }
        }
    }
}
