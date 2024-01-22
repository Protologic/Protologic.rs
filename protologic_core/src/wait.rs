use std::time::Duration;

use crate::constants;

#[cfg(not(mock_protologic))]
#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern
{
    /// Yield execution for one frame
    pub fn sched_yield() -> i32;
}

/// Yield execution for one frame
#[cfg(mock_protologic)]
pub unsafe fn sched_yield() -> i32 { panic!("extern called in test mode!") }

/// Skip one single tick
pub fn wait_tick()
{
    wait_ticks(1);
}

/// Wait for the given number of ticks to pass
pub fn wait_ticks(ticks: u32)
{
    // No need to invalidate the cache if we're not actually waiting
    if ticks == 0 {
        return;
    }

    // Wait for as many ticks as necessary 
    for _ in 0..ticks {
        unsafe { sched_yield(); }
    }

    // Invalidate the cache now that execution is returning to caller code
    unsafe { crate::lowlevel::quickstate::quickstate_invalidate_cache(); }
}

/// Wait for the given amount of time to pass
pub fn wait_time(timespan: Duration)
{
    // We can only wait an exact number of ticks. Round down.
    let tick_count = f32::floor(timespan.as_secs_f32() / constants::tick_duration()) as u32;
    wait_ticks(tick_count);
}