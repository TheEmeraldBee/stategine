# Stategine
An application engine for handling systems that run with shared states and conditions

# Weird name, how do I say it?
It's pronounced `statigen`, didn't want to name it that, because it points at the wrong image for the crate.

# What does it even do?
It's an engine for creating sets of systems with shared states.
For Example, you can turn the following code
```rust
use std::time::Duration;

use ratatui::{
    crossterm::{
        self,
        event::{Event, KeyCode},
    },
    prelude::*,
    widgets::Paragraph,
};

fn main() {
    let mut term = ratatui::init();
    let mut running = true;

    let lines = vec![
        "A: This is ratatui!",
        "B: Cool but challenging!",
        "C: Look at the example!",
        "press `q` to exit",
    ];

    while running {
        term.draw(|f| {
            for (i, line) in lines.iter().enumerate() {
                let rect = Rect::new(0, i as u16, f.area().width, 1);
                f.render_widget(Paragraph::new(*line), rect);
            }
        })
        .unwrap();

        if crossterm::event::poll(Duration::from_millis(500)).unwrap() {
            let event = crossterm::event::read().unwrap();
            if let Event::Key(k) = event {
                if k.code == KeyCode::Char('q') {
                    running = false;
                }
            }
        }
    }

    ratatui::restore();
}
```

Into something much easier to read!

```rust
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
                f.render_widget(Paragraph::new(line.as_str()), rect);
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
```

Although the number of lines required for this simple example increased, large projects heavily benifit!
For example, I am currently writing a terminal UI for a markdown task and note editor. This project was written for it!

# Why did you make this after making Widgetui?
Well, [widgetui](https://github.com/TheEmeraldBee/widgetui) was a really fun project, but was really narrow in usable scope.
As well, now that I have my own tui engine [ascii-forge](https://github.com/TheEmeraldBee/ascii-forge),
I won't personally be using widgetui anymore. So, I made this sytem to allow anyone to use this, even people working on something like games!

# What makes this different from an ECS?
Well, ECSs use concepts where entities can have multiple components. The biggest difference in Stategine is that entities are just structs
The point of this is that you don't really need some heavily complex ECS for a simple project. The goal of this project is to be that inbetween

# How do I use this?
Check out the examples! There are a few examples of how to use the project, and cover almost all of the built in functionallity.

# What if it's missing a feature I need?
If you feel strongly that a feature needs to be added to this engine, please let me know! I would love to be able to help make this the perfect
system for you to use. If you need a feature, or have an idea, create an Issue!
