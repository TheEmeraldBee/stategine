use crate::system::System;
use crate::system::into_system::IntoSystem;

/// Enables a tuple of Widgets to be registered with the widget method.
pub trait IntoSystemSet<T, Inputs, Types> {
    fn into_widget_set(self) -> Vec<Box<dyn for<'a> System<T>>>;
}

macro_rules! impl_into_system_set {
    ($($input:ident $next:ident $widget:ident $func:ident $num:tt)*) => {
        impl<T, $($next,)* $($input,)* $($widget: for<'a> System<T> + 'static,)* $($func),*> IntoSystemSet<T, ($($input,)*), ($($next,)*)> for ($($func,)*)
        where
            $($func: for<'a> IntoSystem<T, $input, $next, System = $widget>),*
        {
            fn into_widget_set(self) -> Vec<Box<dyn for<'a> System<T>>> {
                vec![
                    $(Box::new(self.$num.into_system())),*
                ]
            }
        }
    };
}

impl_into_system_set! {
    I0 N0 W0 F0 0
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
    I9 N9 W9 F9 9
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
    I9 N9 W9 F9 9
    I10 N10 W10 F10 10
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
    I9 N9 W9 F9 9
    I10 N10 W10 F10 10
    I11 N11 W11 F11 11
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
    I9 N9 W9 F9 9
    I10 N10 W10 F10 10
    I11 N11 W11 F11 11
    I12 N12 W12 F12 12
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
    I9 N9 W9 F9 9
    I10 N10 W10 F10 10
    I11 N11 W11 F11 11
    I12 N12 W12 F12 12
    I13 N13 W13 F13 13
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
    I9 N9 W9 F9 9
    I10 N10 W10 F10 10
    I11 N11 W11 F11 11
    I12 N12 W12 F12 12
    I13 N13 W13 F13 13
    I14 N14 W14 F14 14
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
    I9 N9 W9 F9 9
    I10 N10 W10 F10 10
    I11 N11 W11 F11 11
    I12 N12 W12 F12 12
    I13 N13 W13 F13 13
    I14 N14 W14 F14 14
    I15 N15 W15 F15 15
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
    I9 N9 W9 F9 9
    I10 N10 W10 F10 10
    I11 N11 W11 F11 11
    I12 N12 W12 F12 12
    I13 N13 W13 F13 13
    I14 N14 W14 F14 14
    I15 N15 W15 F15 15
    I16 N16 W16 F16 16
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
    I9 N9 W9 F9 9
    I10 N10 W10 F10 10
    I11 N11 W11 F11 11
    I12 N12 W12 F12 12
    I13 N13 W13 F13 13
    I14 N14 W14 F14 14
    I15 N15 W15 F15 15
    I16 N16 W16 F16 16
    I17 N17 W17 F17 17
}
impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
    I9 N9 W9 F9 9
    I10 N10 W10 F10 10
    I11 N11 W11 F11 11
    I12 N12 W12 F12 12
    I13 N13 W13 F13 13
    I14 N14 W14 F14 14
    I15 N15 W15 F15 15
    I16 N16 W16 F16 16
    I17 N17 W17 F17 17
    I18 N18 W18 F18 18
}

impl_into_system_set! {
    I0 N0 W0 F0 0
    I1 N1 W1 F1 1
    I2 N2 W2 F2 2
    I3 N3 W3 F3 3
    I4 N4 W4 F4 4
    I5 N5 W5 F5 5
    I6 N6 W6 F6 6
    I7 N7 W7 F7 7
    I8 N8 W8 F8 8
    I9 N9 W9 F9 9
    I10 N10 W10 F10 10
    I11 N11 W11 F11 11
    I12 N12 W12 F12 12
    I13 N13 W13 F13 13
    I14 N14 W14 F14 14
    I15 N15 W15 F15 15
    I16 N16 W16 F16 16
    I17 N17 W17 F17 17
    I18 N18 W18 F18 18
    I19 N19 W19 F19 19
}
