use crate::lowlevel::{quickstate::{
    get_general_quickstate,
    radar_get_contact_list
}, self};

/// Configure the radar direction and angle, and optionally trigger it
pub fn radar_configure(angle: f32, bearing: f32, elevation: f32, trigger: bool)
{
    radar_set_angle(angle);
    radar_set_bearing(bearing);
    radar_set_elevation(elevation);

    if trigger {
        radar_trigger();
    }
}

/// Set the radar arc angle.
/// Min angle can be retrieved with `const_get_shipradarminangle()`.
/// Max angle can be retrieved with `const_get_shipradarmaxangle()`.
pub fn radar_set_angle(angle: f32)
{
    unsafe
    {
        lowlevel::actions::radar_set_angle(angle);
    }
}

/// Set the radar bearing (0..360)
pub fn radar_set_bearing(bearing: f32)
{
    unsafe
    {
        lowlevel::actions::radar_set_bearing(bearing);
    }
}

/// Set the radar elevation (degrees)
pub fn radar_set_elevation(elevation: f32)
{
    unsafe
    {
        lowlevel::actions::radar_set_elevation(elevation);
    }
}

/// Trigger the radar to scan for targets and return the results in the next frame
pub fn radar_trigger()
{
    unsafe
    {
        lowlevel::actions::radar_trigger();
    }
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

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct RadarGetContactInfo
{
    pub id: i64,
    target_type: i32,
    pub signal_strength: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl RadarGetContactInfo
{
    pub fn get_target_type(&self) -> RadarTargetType
    {
        return self.target_type.into();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadarTargetType
{
    /// This contact is invalid, most likely indicates a bug in the game if you have received this value.
    Invalid,

    SpaceBattleShip,
    SpaceHulk,
    Missile,
    //NuclearShell,
    //Explosion
    Asteroid,
    //VictoryMarker
    FlakShell,
    APShell,
}

impl From<i32> for RadarTargetType
{
    fn from(item: i32) -> Self
    {
        return match item
        {
            0 => RadarTargetType::SpaceBattleShip,
            1 => RadarTargetType::SpaceHulk,
            2 => RadarTargetType::Missile,
            //3 => RadarTargetType::NuclearShell,
            //4 => RadarTargetType::Explosion,
            5 => RadarTargetType::Asteroid,
            //6 => RadarTargetType::VictoryMarker,
            7 => RadarTargetType::FlakShell,
            8 => RadarTargetType::APShell,

            _ => RadarTargetType::Invalid,
        };
    }
}