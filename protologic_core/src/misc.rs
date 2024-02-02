use crate::lowlevel::{self, quickstate::get_general_quickstate};

pub fn self_destruct()
{
    unsafe
    {
        lowlevel::actions::ship_self_destruct();
    }
}

/// Get the unique ID of this vehicle. This is the same ID which is returned in the RADAR contact info for this vehicle.
pub fn vehicle_id() -> i64
{
    return get_general_quickstate().read_i64(680);
}

// /// Turn the ship running light on/off
// pub fn runninglight_set_state(on: bool)
// {
//     unsafe
//     {
//         lowlevel::actions::runninglight_set_state(if on { 1 } else { 0 });
//     }
// }