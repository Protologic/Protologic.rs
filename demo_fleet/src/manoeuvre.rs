use protologic_core::{
    constants,
    maneuvering::*,
    physics::*,
    wait::*
};

pub fn turn_and_stop(x: f32, y: f32, z: f32, ticks: u32)
{
    wheel_set_torque(x, y, z);
    wait_ticks(ticks);
    stop_turning();
}

pub fn stop_turning()
{
    // Loop until we stop turning
    let torque = constants::ship_wheel_torque();
    loop
    {
        let angular_vel = vehicle_get_angular_velocity();
        if f32::abs(angular_vel.0) < 0.001 && f32::abs(angular_vel.1) < 0.001 && f32::abs(angular_vel.2) < 0.001 {
            break;
        }

        // Calculate necessary torque setting to stop in the next tick, this will be clamped to [-1, 1] by the game
        let mass = vehicle_get_mass();
        let x = (angular_vel.0 * mass / torque) / constants::tick_duration();
        let y = (angular_vel.1 * mass / torque) / constants::tick_duration();
        let z = (angular_vel.2 * mass / torque) / constants::tick_duration();
        wheel_set_torque(-x, -y, -z);
        wait_ticks(1);
    }

    // Stop turning
    wheel_set_torque(0.0, 0.0, 0.0);
}