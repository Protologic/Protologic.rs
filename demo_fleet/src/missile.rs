use std::time::Duration;

use rand::{prelude::*, distributions::Uniform};
use protologic_core::{
    highlevel::actions::*,
    utils::*,
    queries::*,
    RadarTargetType, wasi::sched_yield,
};

use crate::{turn_and_stop, stop_turning};

pub fn run()
{
    // Pulse the engines a bit to clear the ship
    engine_set_throttle(1.0);
    wait_time(Duration::from_secs_f32(1f32));
    engine_set_throttle(0.0);

    // Turn in a random direction for a random amount of time
    let mut rng = rand::thread_rng();
    let ticks = rng.gen_range(750..1500u32);
    let xyz = Uniform::new(-1f32, 1f32);
    let x = xyz.sample(&mut rng);
    let y = xyz.sample(&mut rng);
    let z = xyz.sample(&mut rng);
    turn_and_stop(x, y, z, ticks);

    // Fire the engines
    engine_set_throttle(1.0);
    wait_time(Duration::from_secs_f32(5f32));
    engine_set_throttle(0.0);

    // Spin end over end, looking for radar contacts
    wheel_set_torque(0.0, 1.0, 0.0);
    wait_time(Duration::from_secs_f32(4f32));

    // Wait for a contact
    let mut contacts = Vec::new();
    loop {
        wait_ticks(1);

        // Get contacts, skip to next tick if there are none
        radar_get_contacts(&mut contacts);
        if contacts.len() == 0 {
            continue;
        }

        // Get ship contacts, skip to next tick if there are none
        let target = contacts.iter()
            .filter(|x| x.get_target_type() == RadarTargetType::SpaceBattleShip)
            .choose(&mut rng);
        if target.is_none() {
            continue;
        }

        // Target acquired
        let target = target.unwrap();
        println!("Found ship from missile!");

        // Stop turning
        stop_turning();

        // Burn for the target
        engine_set_throttle(1.0);

        // As soon as the target is lost, self destruct
        loop
        {
            radar_get_contacts(&mut contacts);
            let target = contacts.iter()
                .filter(|x| x.get_target_type() == RadarTargetType::SpaceBattleShip)
                .choose(&mut rng);

            if target.is_none() {
                self_destruct();
            }

            wait_ticks(1);
        }
    }
}