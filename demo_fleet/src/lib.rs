use protologic_core::actions::engine_set_throttle;

extern crate protologic_core;
extern crate rand;
extern crate rand_chacha;

mod state;

/// Called by the game every frame
#[no_mangle]
pub extern fn tick()
{
    state::State::get_state_singleton().lock().unwrap().tick();
}

/// Called by the game whenever a "trap" occurs in the tick execution.
#[no_mangle]
pub extern fn trap_handler(trap_code: protologic_core::trap::TrapCode)
{
    // Get the current state (which has just been interrupted by a trap)
    let mut state = state::State::get_state_singleton().lock().unwrap();

    // Call the trap handler on that state. This produces an entirely new state, to allow for error recovery.
    *state = state.trap_handler(trap_code);
}