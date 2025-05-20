use std::{thread::sleep, time::Duration};

use stategine::prelude::*;

struct RunningCounter(i32);

fn render_check(cntr: Res<RunningCounter>) -> bool {
    cntr.0 % 2 == 0
}
fn render(running_state: Res<RunningCounter>) {
    println!("Time Remaining: {}", running_state.0 / 2);
}

fn change_state(mut running_state: ResMut<RunningCounter>) {
    sleep(Duration::from_millis(500));

    running_state.0 -= 1;
}

fn main() {
    let mut engine = Engine::new();
    engine.state(RunningCounter(10));
    engine.system(ConditionalSystemSet::new(render_check, (render,)));
    engine.systems((change_state,));

    engine.entity(5).entity(10);

    loop {
        let running = engine.get_state::<RunningCounter>();
        if running.0 <= 0 {
            break;
        }
        drop(running);

        engine.update();
    }
}
