use crate::AmmoType;
use crate::lowlevel::*;
use crate::lowlevel::queries::*;

/// Get the current amount of "fuel" available for CPU operations within this tick.
pub fn cpu_get_fuel() -> i64
{
    // Note: this does _not_ read from quickstate. We want to get a "live" value every time this is called!
    unsafe
    {
        return queries::cpu_get_fuel();
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

/// Get the world space position of this vehicle
pub fn vehicle_get_position() -> (f32, f32, f32)
{
    return get_general_quickstate().read_vector3(0);
}

/// Get the world space velocity of this vehicle
pub fn vehicle_get_velocity() -> (f32, f32, f32)
{
    return get_general_quickstate().read_vector3(12);
}

/// Get the current mass of this vehicle (hull mass + fuel mass)
pub fn vehicle_get_mass() -> f32
{
    return get_general_quickstate().read_f32(582);
}

/// Get the world space orientation of this vehicle as a quaternion (XYZW)
pub fn vehicle_get_orientation() -> (f32, f32, f32, f32)
{
    return get_general_quickstate().read_quaternion_xyzw(24);
}

/// Get the world space angular velocity of this vehicle
pub fn vehicle_get_angular_velocity() -> (f32, f32, f32)
{
    return get_general_quickstate().read_vector3(40);
}

/// Get the number of contacts detected by the radar last time it was triggered
pub fn radar_get_contact_count() -> i32
{
    return get_general_quickstate().read_i32(52);
}

/// Get all info about all radar targets
pub fn radar_get_contacts(output: &mut Vec<RadarGetContactInfo>)
{
    let pre_count = radar_get_contact_count();
    output.clear();
    output.reserve(pre_count as usize);

    unsafe
    {
        let start = output.as_mut_ptr();
        let count = radar_get_contact_list(start, pre_count);
        assert!(count <= pre_count);
        output.set_len(count as usize);
    }
}

/// Get the current amount of noise received by the RADAR
pub fn radar_get_noise() -> f32
{
    return get_general_quickstate().read_f32(578);
}

/// Get the current bearing of the gun turret with the given index
pub fn gun_get_bearing(index: i32) -> f32
{
    if index < 0 || index >= 4 {
        panic!("Unknown gun index: {}", index)
    }

    return get_general_quickstate().read_f32((56 + index * 4) as usize);
}

/// Get the current elevation of the gun turret with the given index
pub fn gun_get_elevation(index: i32) -> f32
{
    if index < 0 || index >= 4 {
        panic!("Unknown gun index: {}", index)
    }

    return get_general_quickstate().read_f32((152 + index * 4) as usize);
}

/// Get the time before the gun turret with the given index can fire again
pub fn gun_get_refiretime(index: i32) -> f32
{
    if index < 0 || index >= 4 {
        panic!("Unknown gun index: {}", index)
    }

    return get_general_quickstate().read_f32((244 + index * 4) as usize);
}

/// Get the capacity of the current magazine for the gun turret with the given index
pub fn gun_get_magazine_capacity(index: i32) -> i32
{
    if index < 0 || index >= 4 {
        panic!("Unknown gun index: {}", index)
    }

    return get_general_quickstate().read_u16((428 + index * 2) as usize).into();
}

/// Get the number of shots in the magazine for the gun turret with the given index
pub fn gun_get_magazine_remaining(index: i32) -> i32
{
    if index < 0 || index >= 4 {
        panic!("Unknown gun index: {}", index)
    }

    return get_general_quickstate().read_u16((474 + index * 2) as usize).into();
}

/// Get the type of ammo loaded in the magazine for the gun turret with the given index
pub fn gun_get_magazine_type(index: i32) -> AmmoType
{
    if index < 0 || index >= 4 {
        panic!("Unknown gun index: {}", index)
    }

    return get_general_quickstate().read_u16((520 + index * 2) as usize).into();
}

/// Get the amount of time remaining before the current reload is completed for the gun turret with the given index
pub fn gun_get_magazine_reloadtime(index: i32) -> f32
{
    if index < 0 || index >= 4 {
        panic!("Unknown gun index: {}", index)
    }

    return get_general_quickstate().read_f32((336 + index * 4) as usize);
}

/// Get the total number of unfired missiles remaining 
pub fn missilelauncher_get_stockpile() -> u16
{
    return get_general_quickstate().read_u16(586);
}

/// Get the amount of time remaining before the current reload is completed for the missile launcher turret with the given index
pub fn missilelauncher_get_reloadtime(index: i32) -> f32
{
    if index < 0 || index > crate::constants::ship_missile_launcher_count() {
        panic!("Unknown missile launcher: {}", index)
    }

    return get_general_quickstate().read_f32((588 + index * 4) as usize);
}