use std::time::Duration;

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

/// skip one single tick
pub fn wait_tick()
{
    unsafe
    {
        sched_yield();
        crate::lowlevel::queries::quickstate_invalidate_cache();
    }
}

/// Wait for the given number of ticks to pass
pub fn wait_ticks(ticks: u32)
{
    for _ in 0..ticks {
        wait_tick();
    }
}

/// Wait for the given amount of time to pass
pub fn wait_time(timespan: Duration)
{
    let tick_count = f32::floor(timespan.as_secs_f32() / crate::constants::tick_duration()) as u32;
    wait_ticks(tick_count);
}