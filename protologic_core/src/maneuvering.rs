use crate::lowlevel::{
    self,
    quickstate::get_general_quickstate
};

/// Set the rocket engine throttle
pub fn engine_set_throttle(throttle: f32)
{
    unsafe
    {
        lowlevel::actions::engine_set_throttle(throttle);
    }
}

/// Get the amount of rocket fuel currently in the tank
pub fn engine_get_fuel_amount() -> f32
{
    return get_general_quickstate().read_f32(566);
}

/// Get the max amount of rocket fuel the tank can hold
pub fn engine_get_fuel_capacity() -> f32
{
    return get_general_quickstate().read_f32(570);
}

/// Get the current engine throttle
pub fn engine_get_throttle() -> f32
{
    return get_general_quickstate().read_f32(574);
}

/// Get the thrust at max throttle
pub fn engine_get_max_thrust() -> f32
{
    return get_general_quickstate().read_f32(664);
}

/// Get the thrust at max throttle
pub fn engine_get_max_fuel_consumption() -> f32
{
    return get_general_quickstate().read_f32(668);
}

// Set the torque throttle for each axis
pub fn wheel_set_torque(x: f32, y: f32, z: f32)
{
    unsafe
    {
        lowlevel::actions::wheel_set_torque(x, y, z);
    }
}