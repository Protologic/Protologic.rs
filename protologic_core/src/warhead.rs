use crate::lowlevel::quickstate::get_general_quickstate;

/// Immediately self destruct this vehicle
pub fn self_destruct()
{
    unsafe
    {
        self::inner::ship_self_destruct();
    }
}

/// Arm the warhead on the current vehicle, this means that the warhead will explode when this vehicle is destroyed
pub fn warhead_arm()
{
    unsafe
    {
        self::inner::warhead_arm();
    }
}

/// Check if the current vehicle has a warhead that can be armed
pub fn vehicle_has_warhead() -> bool
{
    return get_general_quickstate().read_i32(688) == 1;
}

mod inner
{
    protologic_define_extern!(pub(crate) fn warhead_arm());
    protologic_define_extern!(pub(crate) fn ship_self_destruct());
}