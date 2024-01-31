use crate::lowlevel::{quickstate::{
    get_general_quickstate,
    radar_get_contact_list2
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
    // Presize the list to at least as many contacts as we're about to receive
    let pre_count = radar_get_contact_count();
    output.clear();
    output.reserve(pre_count as usize);

    // Do nothing if there are no contacts
    if pre_count == 0 {
        return;
    }

    // Read the contact data
    unsafe
    {
        let start = output.as_mut_ptr();
        let count = radar_get_contact_list2(start, pre_count, std::mem::size_of::<RadarGetContactInfo>() as i32);
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
    pub target_type: RadarTargetType,
    pub signal_strength: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ToPrimitive, FromPrimitive)]
#[repr(i32)]
pub enum RadarTargetType
{
    /// This contact is invalid, most likely indicates a bug in the game if you have received this value.
    Invalid = -1,

    SpaceBattleShip = 0,
    SpaceHulk = 1,
    Missile = 2,
    //NuclearShell = 3,
    //Explosion = 4
    Asteroid = 5,
    //VictoryMarker = 6
    FlakShell = 7,
    APShell = 8,
}