use std::time::Duration;

//protologic_define_extern!(pub(crate) fn black_box_yield1(_0: *mut u8) -> i32);
protologic_define_extern!(pub(crate) fn black_box_yield2(_0: *mut u8, _1: *mut u8) -> i32);
//protologic_define_extern!(pub(crate) fn black_box_yield3(_0: *mut u8, _1: *mut u8, _2: *mut u8) -> i32);
//protologic_define_extern!(pub(crate) fn black_box_yield4(_0: *mut u8, _1: *mut u8, _2: *mut u8, _3: *mut u8) -> i32);
//protologic_define_extern!(pub(crate) fn black_box_yield5(_0: *mut u8, _1: *mut u8, _2: *mut u8, _3: *mut u8, _4: *mut u8) -> i32);

//protologic_define_extern!(pub(crate) fn black_box_noop1(_0: *mut u8) -> i32);
//protologic_define_extern!(pub(crate) fn black_box_noop2(_0: *mut u8, _1: *mut u8) -> i32);
//protologic_define_extern!(pub(crate) fn black_box_noop3(_0: *mut u8, _1: *mut u8, _2: *mut u8) -> i32);
//protologic_define_extern!(pub(crate) fn black_box_noop4(_0: *mut u8, _1: *mut u8, _2: *mut u8, _3: *mut u8) -> i32);
//protologic_define_extern!(pub(crate) fn black_box_noop5(_0: *mut u8, _1: *mut u8, _2: *mut u8, _3: *mut u8, _4: *mut u8) -> i32);

/// skip one single tick
pub fn wait_tick()
{
    unsafe
    {
        // We need to pass a pointer to every buffer that _might_ be modified, to convince
        // the compiler that they might be changed as part of the "yield" call. Otherwise
        // optimisations might inline two reads from the same pointer on either side of this
        // call. In theory this should be done with volatile reads, but this seems more reliable.
        black_box_yield2(
            crate::lowlevel::queries::get_quickstate_buffer_pointer(),
            crate::radio::get_radio_buffer_pointer()
        );
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