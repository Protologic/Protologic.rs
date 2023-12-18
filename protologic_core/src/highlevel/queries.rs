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
    return queries::get_quickstate().read_engine_fuelamount();
}

/// Get the max amount of rocket fuel the tank can hold
pub fn engine_get_fuel_capacity() -> f32
{
    return queries::get_quickstate().read_engine_fuelcapacity();
}

/// Get the current engine throttle
pub fn engine_get_throttle() -> f32
{
    return queries::get_quickstate().read_engine_throttle();
}

/// Get the world space position of this ship
pub fn ship_get_position() -> (f32, f32, f32)
{
    return queries::get_quickstate().read_position();
}

/// Get the world space velocity of this ship
pub fn ship_get_velocity() -> (f32, f32, f32)
{
    return get_quickstate().read_velocity();
}

/// Get the world space orientation of this ship as a quaternion (WXYZ)
pub fn ship_get_orientation() -> (f32, f32, f32, f32)
{
    return get_quickstate().read_orientation();
}

/// Get the world space angular velocity of this ship
pub fn ship_get_angular_velocity() -> (f32, f32, f32)
{
    return get_quickstate().read_angular_velocity();
}

/// Get the number of contacts detected by the radar last time it was triggered
pub fn radar_get_contact_count() -> i32
{
    return get_quickstate().read_radar_contact_count();
}

/// Get all info about all radar targets
pub fn radar_get_contacts(output: &mut Vec<RadarGetContactInfo>)
{
    let count = radar_get_contact_count();
    output.clear();
    output.reserve(count as usize);

    unsafe
    {
        let start = output.as_mut_ptr();
        let count = radar_get_contact_list(start, count);
        output.set_len(count as usize);
    }
}

/// Get the current amount of noise received by the RADAR
pub fn radar_get_noise() -> f32
{
    return get_quickstate().read_radar_noise();
}

/// Get the current bearing of the gun turret with the given index
pub fn gun_get_bearing(index: i32) -> f32
{
    return get_quickstate().read_gun_bearing(index);
}

/// Get the current elevation of the gun turret with the given index
pub fn gun_get_elevation(index: i32) -> f32
{
    return get_quickstate().read_gun_elevation(index);
}

/// Get the time before the gun turret with the given index can fire again
pub fn gun_get_refiretime(index: i32) -> f32
{
    return get_quickstate().read_gun_refiretime(index);
}

/// Get the capacity of the current magazine for the gun turret with the given index
pub fn gun_get_magazine_capacity(index: i32) -> i32
{
    return get_quickstate().read_gun_magazinecapacity(index).into();
}

/// Get the number of shots in the magazine for the gun turret with the given index
pub fn gun_get_magazine_remaining(index: i32) -> i32
{
    return get_quickstate().read_gun_magazineremaining(index).into();
}

/// Get the type of ammo loaded in the magazine for the gun turret with the given index
pub fn gun_get_magazine_type(index: i32) -> AmmoType
{
    return get_quickstate().read_gun_magazinetype(index);
}

/// Get the amount of time remaining before the current reload is completed for the gun turret with the given index
pub fn gun_get_magazine_reloadtime(index: i32) -> f32
{
    return get_quickstate().read_gun_reloadtime(index);
}