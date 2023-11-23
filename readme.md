## Protologic.rs

This is a template project for using [Rust](https://www.rust-lang.org/) in [Protologic](https://github.com/Protologic).

## Getting Started

1. Clone this repository
2. Install Rust `wasm` toolchain:
   1. Open terminal (e.g. powershell/bash):
   2. Run: `rustup target add wasm32-wasi`
   3. Run: `rustup target add wasm32-unknown-unknown`
3. Open `demo_fleet` folder, this contains a simple demonstration fleet.
4. Run `build.ps1` to compile the demo fleet.
5. todo: sim instructions/link
6. todo: player instructions/link

## Project Structure

The project is split into two Rust "crates".

### protologic_core

`protologic_core` contains the bindings to the game API. `lowlevel` are the direct bindings to the unsafe WASM API. `highlevel` are slightly nicer to use wrappers around the low level bindings. You should never need to edit this project.

### demo_fleet

`demo_fleet` contains a simple demo fleet. `lib.rs` is the root of this project and contains 2 functions which the game will call. Your fleet **must** implement both of these methods!

The `main` method is called by the game every frame.
```rust
#[no_mangle]
pub extern fn main()
{
    // Every frame
}
```

The `trap_handler` method is called is a fatal trap terminates execution of your program (e.g. a `panic`). You can use this to recover from errors, for example the `demo_fleet` simply recreates a new `State` to replace the old one.
```rust
#[no_mangle]
pub extern fn trap_handler(trap_code: protologic_core::trap::TrapCode)
{
    // Something went wrong!
}
```

`state.rs` is the main implementation of the demo fleet. You can delete all of this, or you can use it as an example framework to get started.

## Special Methods

If your code calls the special method `sched_yield()` then execution of your program will be _immediately_ suspended, execution will resume from that point next frame instead of calling `main()`. This allows you to write simpler programs, for example:

```rust
radar_trigger(); // Trigger radar, results will be available next frame
sched_yield(); // Wait for the next frame.
for i in 0..radar_get_target_count() {
    // Radar contact!
}
```