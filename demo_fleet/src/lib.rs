extern crate protologic_core;
extern crate rand;
extern crate rand_chacha;

use std::sync::{OnceLock, Mutex};

mod state;
static STATE: OnceLock<Mutex<state::State>> = OnceLock::new();

/// Called by the game every frame
#[no_mangle]
pub extern fn main()
{
    get_state().lock().unwrap().main();
}

/// Called by the game whenever a "trap" occurs in the main execution.
#[no_mangle]
pub extern fn trap_handler(trap_code: protologic_core::trap::TrapCode)
{
    // Get the current state (which has just been interrupted by a trap)
    let mut state = get_state().lock().unwrap();

    // Call the trap handler on that state. This produces an entirely new state, to allow for error recovery.
    *state = state.trap_handler(trap_code);
}

/// The "state" of the program is stored in a mutable static object. Get access to the state through this method.
/// This allows state to persist between calls of main/trap_handler.
fn get_state() -> &'static Mutex<state::State>
{
    return STATE.get_or_init(|| Mutex::new(state::State::new(0)));
}