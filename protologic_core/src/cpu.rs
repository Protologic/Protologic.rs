use crate::lowlevel::quickstate;

/// Get the current amount of "fuel" available for CPU operations within this tick.
pub fn cpu_get_fuel() -> i64
{
    // Note: this does _not_ read from quickstate. We want to get a "live" value every time this is called!
    unsafe
    {
        return quickstate::cpu_get_fuel();
    }
}