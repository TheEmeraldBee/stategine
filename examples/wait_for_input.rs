use std::{thread::sleep, time::Duration};

use stategine::{
    Engine,
    system::{
        SystemResult,
        into_system::IntoSystem,
        param::{query_mut::QueryMut, res::Res, res_mut::ResMut},
    },
};

struct RunningCounter(i32);

fn render(running_state: Res<RunningCounter>) -> SystemResult<()> {
    if running_state.0 % 2 == 0 {
        println!("Time Remaining: {}", running_state.0 / 2);
    }
    Ok(())
}

fn change_state(mut running_state: ResMut<RunningCounter>) -> SystemResult<()> {
    sleep(Duration::from_millis(500));

    running_state.0 -= 1;
    Ok(())
}

fn main() {
    let mut engine = Engine::new();
    engine.systems((render, change_state));
    engine.state(RunningCounter(8));

    engine.entity(5).entity(10);

    loop {
        let running = engine.get_state::<RunningCounter>();
        if running.0 <= 0 {
            break;
        }
        drop(running);

        engine.update().unwrap();
    }
}
