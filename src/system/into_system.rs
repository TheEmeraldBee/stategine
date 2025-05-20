use crate::system::System;

/// Allows all versions of a function with the parameters
/// that work within a widget to be converted into a widget
pub trait IntoSystem<T, Input, Data> {
    type System: System<T>;

    fn into_widget(self) -> Self::System;
}
