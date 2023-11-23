use crate::lowlevel;

/// Yield execution until the next frame
pub fn sched_yield() -> bool
{
    unsafe
    {
        return lowlevel::wasi::sched_yield() == 0;
    }
}