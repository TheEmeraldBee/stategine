use std::{io::Stdout, time::Duration};

use ratatui::{
    crossterm::{
        self,
        event::{Event, KeyCode},
    },
    prelude::*,
    widgets::Paragraph,
};
use stategine::prelude::*;

type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;
struct Running(bool);

fn ui(mut terminal: ResMut<Terminal>, lines: Query<String>) {
    terminal
        .draw(|f| {
            for (i, line) in lines.iter().enumerate() {
                let rect = Rect::new(0, i as u16, f.area().width, 1);
                f.render_widget(Paragraph::new(line.1.as_str()), rect);
            }
        })
        .unwrap();
}

fn exit_condition(mut running: ResMut<Running>) {
    if crossterm::event::poll(Duration::from_millis(500)).unwrap() {
        let event = crossterm::event::read().unwrap();
        if let Event::Key(k) = event {
            if k.code == KeyCode::Char('q') {
                running.0 = false;
            }
        }
    }
}

fn main() {
    let term: Terminal = ratatui::init();

    let mut engine = Engine::new();
    engine.states((term, Running(true)));
    engine
        .entity("A: This is ratatui!".to_string())
        .entity("B: But running with stategine".to_string())
        .entity("C: Making it less complicated!".to_string())
        .entity("Press `q` to quit".to_string());
    engine.systems((ui, exit_condition));

    while engine.get_state::<Running>().0 {
        engine.update();
    }

    ratatui::restore();
}
