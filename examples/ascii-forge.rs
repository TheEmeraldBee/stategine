use std::time::Duration;

use ascii_forge::prelude::*;
use stategine::prelude::*;

fn render_hello(mut window: ResMut<Window>) {
    render!(window, (0, 0) => [ "Hello".red(), ", ".blue(), "World".green(), "!".grey() ]);
}

fn update_window(mut window: ResMut<Window>) {
    window.update(Duration::from_millis(500)).unwrap();
}

fn should_exit(window: Res<Window>, mut running: ResMut<Running>) {
    if event!(window, Event::Key(k) => k.code == KeyCode::Char('q')) {
        running.0 = false;
    }
}

struct Running(bool);

fn main() {
    let window = Window::init().unwrap();
    handle_panics();

    let mut engine = Engine::new();
    engine.states((window, Running(true)));
    engine.systems((render_hello, update_window, should_exit));

    while engine.get_state::<Running>().0 {
        engine.update();
    }
}
