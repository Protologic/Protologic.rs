use crate::lowlevel;

pub fn self_destruct()
{
    unsafe
    {
        lowlevel::actions::ship_self_destruct();
    }
}

/// Turn the ship running light on/off
pub fn runninglight_set_state(on: bool)
{
    unsafe
    {
        lowlevel::actions::runninglight_set_state(if on { 1 } else { 0 });
    }
}