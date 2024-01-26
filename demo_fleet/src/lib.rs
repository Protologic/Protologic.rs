use std::env;

extern crate protologic_core;
extern crate rand;
extern crate rand_chacha;

mod ship;
mod missile;
mod radio;
mod manoeuvre;

/// Called by the game every frame
#[no_mangle]
pub extern fn tick()
{
    // Print all environment variables
    for (k, v) in env::vars() {
        println!("ENV: {} = {}", k, v);
    }

    // This might be a missile or a ship.
    // Check which one we are and run the appropriate code.
    let entity_type = env::var("Type").unwrap();
    match entity_type.as_str()
    {
        "Missile" => missile::run(),
        "Ship" => ship::run(),

        _ => panic!("Unknown type! {}", entity_type),
    }
}