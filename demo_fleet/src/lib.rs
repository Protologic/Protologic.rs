use std::env;

extern crate protologic_core;
extern crate rand;
extern crate rand_chacha;

mod ship;
mod missile;
mod radio;

use protologic_core::{
    highlevel::actions::*,
    wait::*,
    queries::*,
    constants::*,
};

/// Called by the game every frame
#[no_mangle]
pub extern fn tick()
{
    // Print all environment variables
    for (k, v) in env::vars() {
        println!("ENV: {} = {}", k, v);
    }

    // This might be a missile or a ship.
    // Check which one we are and run the appropriate code.
    let entity_type = env::var("Type").unwrap();
    match entity_type.as_str()
    {
        "Missile" => missile::run(),
        "Ship" => ship::run(),

        _ => panic!("Unknown type! {}", entity_type),
    }
}

fn turn_and_stop(x: f32, y: f32, z: f32, ticks: u32)
{
    wheel_set_torque(x, y, z);
    wait_ticks(ticks);
    stop_turning();
}

fn stop_turning()
{
    // Loop until we stop turning
    let torque = ship_wheel_torque();
    loop
    {
        let angular_vel = ship_get_angular_velocity();
        if f32::abs(angular_vel.0) < 0.001 && f32::abs(angular_vel.1) < 0.001 && f32::abs(angular_vel.2) < 0.001 {
            break;
        }

        // Calculate necessary torque setting to stop in the next tick, this will be clamped to [-1, 1] by the game
        let mass = ship_get_mass();
        let x = (angular_vel.0 * mass / torque) / tick_duration();
        let y = (angular_vel.1 * mass / torque) / tick_duration();
        let z = (angular_vel.2 * mass / torque) / tick_duration();
        wheel_set_torque(-x, -y, -z);
        wait_ticks(1);
    }

    // Stop turning
    wheel_set_torque(0.0, 0.0, 0.0);
}