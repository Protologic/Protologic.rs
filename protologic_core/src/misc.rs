use crate::lowlevel::quickstate::get_general_quickstate;

/// Get the unique ID of this vehicle. This is the same ID which is returned in the RADAR contact info for this vehicle.
pub fn vehicle_id() -> i64
{
    return get_general_quickstate().read_i64(680);
}