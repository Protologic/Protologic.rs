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
        ship_get_position_ptr((&mut p) as *mut f32);
    }
    return (p[0], p[1], p[2]);
}

/// Get the world space velocity of this ship
pub fn ship_get_velocity() -> (f32, f32, f32)
{
    let mut v = [0.0, 0.0, 0.0];
    unsafe
    {
        ship_get_velocity_ptr((&mut v) as *mut f32);
    }
    return (v[0], v[1], v[2]);
}

/// Get the world space orientation of this ship as a quaternion (WXYZ)
pub fn ship_get_orientation() -> (f32, f32, f32, f32)
{
    let mut q = [0.0, 0.0, 0.0, 0.0];
    unsafe
    {
        ship_get_orientation_ptr((&mut q) as *mut f32);
    }
    return (q[0], q[1], q[2], q[3]);
}

/// Get the world space angular velocity of this ship
pub fn ship_get_angular_velocity() -> (f32, f32, f32)
{
    let mut a = [0.0, 0.0, 0.0];
    unsafe
    {
        ship_get_angularvelocity_ptr((&mut a) as *mut f32);
    }
    return (a[0], a[1], a[2]);
}

/// Get the number of targets detected by the radar last time it was triggered
pub fn radar_get_target_count() -> i32
{
    unsafe
    {
        lowlevel::queries::radar_get_target_count()
    }
}

/// Get the target ID, type and distance for a given radar detection
pub fn radar_get_target(index: i32) -> (i64, RadarTargetType, f32)
{
    unsafe
    {
        let mut p: RadarGetTargetInfo = RadarGetTargetInfo::default();
        lowlevel::queries::radar_get_target_info(index, (&mut p) as *mut RadarGetTargetInfo);

        return (
            p.id,
            RadarTargetType::from_raw(p.target_type),
            p.distance
        );
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum RadarTargetType
{
    Unknown,

    SpaceBattleShip,
    SpaceHulk,
    Missile,
    Shell ,
    Asteroid,
}

impl RadarTargetType
{
    fn from_raw(value: i32) -> RadarTargetType
    {
        return match value
        {
            0 => RadarTargetType::SpaceBattleShip,
            1 => RadarTargetType::SpaceHulk,
            2 => RadarTargetType::Missile,
            3 => RadarTargetType::Shell,
            5 => RadarTargetType::Asteroid,
            _ => RadarTargetType::Unknown
        };
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
