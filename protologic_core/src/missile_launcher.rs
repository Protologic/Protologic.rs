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

pub fn missilelauncher_configure(index: i32, engine: MissileEngineType, warhead: MissileWarheadType)
{
    unsafe
    {
        lowlevel::actions::missilelauncher_set_enginetype(index, engine.into());
        lowlevel::actions::missilelauncher_set_warheadtype(index, warhead.into());
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissileEngineType
{
    LowThrust,
    MedThrust,
    HighThrust
}

impl Into<i32> for MissileEngineType
{
    fn into(self) -> i32
    {
        return match self
        {
            MissileEngineType::LowThrust => 0,
            MissileEngineType::MedThrust => 1,
            MissileEngineType::HighThrust => 2
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissileWarheadType
{
    Nuclear,
}

impl Into<i32> for MissileWarheadType
{
    fn into(self) -> i32
    {
        return match self
        {
            MissileWarheadType::Nuclear => 0,
        };
    }
}