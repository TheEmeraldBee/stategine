use std::time::Duration;

use ascii_forge::prelude::*;
use stategine::{prelude::*, system::into_system::IntoSystem};

fn render_hello(mut window: ResMut<Window>) {
    render!(window, (0, 0) => [ "Hello".red(), ", ".blue(), "World".green(), "!".grey() ]);
}

fn update_window(mut window: ResMut<Window>) {
    window.update(Duration::from_millis(500)).unwrap();
}

fn should_exit(window: Res<Window>) -> bool {
    if event!(window, Event::Key(k) => k.code == KeyCode::Char('q')) {
        true
    } else {
        false
    }
}

fn main() {
    let window = Window::init().unwrap();
    handle_panics();

    let mut engine = Engine::new();
    engine.states((window,));
    engine.systems((render_hello, update_window));

    while !engine.oneshot_system::<bool>(should_exit.into_system()) {
        engine.update();
    }
}
