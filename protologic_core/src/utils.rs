use std::time::Duration;

use crate::wasi::sched_yield;

/// Wait for the given number of ticks to pass
pub fn wait_ticks(ticks: u32)
{
    for _ in 0..ticks {
        sched_yield();
    }
}

/// Wait for the given amount of time to pass
pub fn wait_time(timespan: Duration)
{
    let tick_count = f32::floor(timespan.as_secs_f32() / crate::constants::tick_duration()) as u32;
    wait_ticks(tick_count);
}