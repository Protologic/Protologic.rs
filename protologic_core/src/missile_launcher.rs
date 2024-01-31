use num_traits::{ FromPrimitive, ToPrimitive };

use crate::lowlevel::{
    self,
    quickstate::get_general_quickstate
};

pub fn missilelauncher_trigger(index: i32)
{
    unsafe
    {
        lowlevel::actions::missilelauncher_trigger(index);
    }
}

/// Configure a missile launcher cell
/// - index: cell to configure
/// - engine: engine type to install on the missile
/// - warhead: warhead type to install on the missile
/// - fuel_load: How much to fill the fuel tanks (0 to 1)
pub fn missilelauncher_configure(index: i32, engine: MissileEngineType, warhead: MissileWarheadType, fuel_load: f32)
{
    unsafe
    {
        lowlevel::actions::missilelauncher_set_enginetype(index, engine.to_i32().expect("Unknown MissileEngineType"));
        lowlevel::actions::missilelauncher_set_warheadtype(index, warhead.to_i32().expect("Unknown MissileWarheadType"));
        lowlevel::actions::missilelauncher_set_fuelload(index, fuel_load);
    }
}

/// Get the currently configured engine type for a missile launcher cell
/// - index: cell to query
pub fn missilelauncher_get_enginetype(index: i32) -> MissileEngineType
{
    unsafe
    {
        return MissileEngineType::from_i32(lowlevel::actions::missilelauncher_get_enginetype(index)).expect("Unknown MissileEngineType");
    }
}

/// Get the currently configured warhead type for a missile launcher cell
/// - index: cell to query
pub fn missilelauncher_get_warheadtype(index: i32) -> MissileWarheadType
{
    unsafe
    {
        return MissileWarheadType::from_i32(lowlevel::actions::missilelauncher_get_warheadtype(index)).expect("Unknown MissileWarheadType");
    }
}

/// Get the currently configured fuel load (How much to fill the fuel tanks from 0 to 1) for a missile launcher cell
/// - index: cell to query
pub fn missilelauncher_get_fuelload(index: i32) -> f32
{
    unsafe
    {
        return lowlevel::actions::missilelauncher_get_fuelload(index);
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum MissileEngineType
{
    LowThrust,
    MedThrust,
    HighThrust
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum MissileWarheadType
{
    Nuclear = 0,
    Inert = 1,
    Flak = 2,
    Jammer = 3,
}