use crate::lowlevel::{
    actions::*,
    quickstate::get_general_quickstate
};

pub fn gun_configure(index: i32, bearing: f32, elevation: f32, fuse: f32)
{
    gun_set_bearing(index, bearing);
    gun_set_elevation(index, elevation);
    gun_set_fuse(index, fuse);
}

/// Set the bearing the gun with the given index should begin turning to
pub fn gun_set_bearing(index: i32, bearing: f32)
{
    unsafe
    {
        match index
        {
            0 => gun0_set_bearing(bearing),
            1 => gun1_set_bearing(bearing),
            2 => gun2_set_bearing(bearing),
            3 => gun3_set_bearing(bearing),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
}

/// Set the elevation the gun with the given index should begin turning to.
/// Maximum elevation can be retrieved with `const_get_turretmaxelevation()`.
pub fn gun_set_elevation(index: i32, elevation: f32)
{
    unsafe
    {
        match index
        {
            0 => gun0_set_elevation(elevation),
            1 => gun1_set_elevation(elevation),
            2 => gun2_set_elevation(elevation),
            3 => gun3_set_elevation(elevation),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
}

/// Set the fuse for shells fired from the given gun
pub fn gun_set_fuse(index: i32, fuse: f32)
{
    unsafe
    {
        match index
        {
            0 => gun0_set_fuse(fuse),
            1 => gun1_set_fuse(fuse),
            2 => gun2_set_fuse(fuse),
            3 => gun3_set_fuse(fuse),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
}

/// Set a flag to fire the gun with the given index once it is ready to fire
pub fn gun_trigger(index: i32)
{
    unsafe
    {
        match index
        {
            0 => gun0_trigger(),
            1 => gun1_trigger(),
            2 => gun2_trigger(),
            3 => gun3_trigger(),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
}

/// Begin reloading the gun with the given index to a specific ammo type
pub fn gun_reload(index: i32, ammo: AmmoType)
{
    unsafe
    {
        match index
        {
            0 => gun0_reload(ammo.into()),
            1 => gun1_reload(ammo.into()),
            2 => gun2_reload(ammo.into()),
            3 => gun3_reload(ammo.into()),
            _ => panic!("Unknown gun index: {}", index),
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmmoType
{
    Flak,
    ArmourPiercing,

    Unknown
}

impl From<i32> for AmmoType
{
    fn from(item: i32) -> Self
    {
        return match item
        {
            0 => AmmoType::Flak,
            1 => AmmoType::ArmourPiercing,
            _ => AmmoType::Unknown,
        };
    }
}

impl From<u16> for AmmoType
{
    fn from(item: u16) -> Self
    {
        return match item
        {
            0 => AmmoType::Flak,
            1 => AmmoType::ArmourPiercing,
            _ => AmmoType::Unknown,
        };
    }
}

impl Into<i32> for AmmoType
{
    fn into(self) -> i32
    {
        return match self
        {
            AmmoType::Flak => 0,
            AmmoType::ArmourPiercing => 1,
            AmmoType::Unknown => i32::MAX
        };
    }
}