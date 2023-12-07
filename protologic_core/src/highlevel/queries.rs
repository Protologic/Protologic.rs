use crate::lowlevel::{*, self};
use crate::lowlevel::queries::*;

/// Get the current amount of "fuel" available for CPU operations within this tick.
pub fn cpu_get_fuel() -> i64
{
    unsafe
    {
        return queries::cpu_get_fuel();
    }
}

/// Get the world space position of this ship
pub fn ship_get_position() -> (f32, f32, f32)
{
    let mut p = [0.0, 0.0, 0.0];
    unsafe
    {
        ship_get_position_ptr((&mut p) as *mut [f32; 3]);
    }
    return (p[0], p[1], p[2]);
}

/// Get the world space velocity of this ship
pub fn ship_get_velocity() -> (f32, f32, f32)
{
    let mut v = [0.0, 0.0, 0.0];
    unsafe
    {
        ship_get_velocity_ptr((&mut v) as *mut [f32; 3]);
    }
    return (v[0], v[1], v[2]);
}

/// Get the world space orientation of this ship as a quaternion (WXYZ)
pub fn ship_get_orientation() -> (f32, f32, f32, f32)
{
    let mut q = [0.0, 0.0, 0.0, 0.0];
    unsafe
    {
        ship_get_orientation_ptr((&mut q) as *mut [f32; 4]);
    }
    return (q[0], q[1], q[2], q[3]);
}

/// Get the world space angular velocity of this ship
pub fn ship_get_angular_velocity() -> (f32, f32, f32)
{
    let mut a = [0.0, 0.0, 0.0];
    unsafe
    {
        ship_get_angularvelocity_ptr((&mut a) as *mut [f32; 3]);
    }
    return (a[0], a[1], a[2]);
}

/// Get the number of contacts detected by the radar last time it was triggered
pub fn radar_get_contact_count() -> i32
{
    unsafe
    {
        return lowlevel::queries::radar_get_contact_count();
    }
}

/// Get the the approximate position of the contact with the given index (Z element)
pub fn radar_get_contact(index: i32) -> RadarGetContactInfo
{
    unsafe
    {
        let mut p: RadarGetContactInfo = RadarGetContactInfo::default();
        lowlevel::queries::radar_get_contact_info(index, (&mut p) as *mut RadarGetContactInfo);
        return p;
    }
}

/// Get all info about all radar targets
pub fn radar_get_contacts(output: &mut Vec<RadarGetContactInfo>)
{
    let count = radar_get_contact_count();
    output.reserve(count as usize);

    unsafe
    {
        let start = output.as_mut_ptr();
        let count = radar_get_contact_list(start, count);
        output.set_len(count as usize);
    }
}

/// Get the number of targets detected by the radar last time it was triggered
#[deprecated(since="new_radar", note="please use `radar_get_contact_count` instead")]
pub fn radar_get_target_count() -> i32
{
    unsafe
    {
        lowlevel::queries::radar_get_target_count()
    }
}

/// Get the target ID, type and distance for a given radar detection
#[deprecated(since="new_radar", note="please use `radar_get_contact_info` instead")]
pub fn radar_get_target(index: i32) -> RadarGetTargetInfo
{
    unsafe
    {
        let mut p: RadarGetTargetInfo = RadarGetTargetInfo::default();
        lowlevel::queries::radar_get_target_info(index, (&mut p) as *mut RadarGetTargetInfo);
        return p;
    }
}

/// Copy all radar contacts into the given vec
#[deprecated(since="new_radar", note="please use `radar_get_contact_count` instead")]
pub fn radar_get_targets(output: &mut Vec<RadarGetTargetInfo>)
{
    let count = radar_get_target_count();
    output.reserve(count as usize);

    unsafe
    {
        let start = output.as_mut_ptr();
        let count = radar_get_target_list(start, count);
        output.set_len(count as usize);
    }
}

/// Get the current bearing of the gun turret with the given index
pub fn gun_get_bearing(index: i32) -> f32
{
    unsafe
    {
        match index
        {
            0 => gun0_get_bearing(),
            1 => gun1_get_bearing(),
            2 => gun2_get_bearing(),
            3 => gun3_get_bearing(),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
}

/// Get the current elevation of the gun turret with the given index
pub fn gun_get_elevation(index: i32) -> f32
{
    unsafe
    {
        match index
        {
            0 => gun0_get_elevation(),
            1 => gun1_get_elevation(),
            2 => gun2_get_elevation(),
            3 => gun3_get_elevation(),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
}

/// Get the time before the gun turret with the given index can fire again
pub fn gun_get_refiretime(index: i32) -> f32
{
    unsafe
    {
        match index
        {
            0 => gun0_get_refiretime(),
            1 => gun1_get_refiretime(),
            2 => gun2_get_refiretime(),
            3 => gun3_get_refiretime(),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
}
