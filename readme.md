## Protologic.rs

This is a template project for using [Rust](https://www.rust-lang.org/) in [Protologic](https://github.com/Protologic).

## Getting Started

1. Clone this repository
2. Install the `wasm` toolchain for Rust:
   1. Open terminal (e.g. PowerShell/bash):
   2. Run: `rustup target add wasm32-wasi`
3. Download the latest [`binaryen`](https://github.com/WebAssembly/binaryen/releases/) release, place it in `tools/binaryen`
5. Run `build.ps1` to compile the demo fleet.
6. todo: sim instructions/link
7. todo: player instructions/link

### Alternative

If you do not require the entire example project `protologic_core` is also available on [crates.io](https://crates.io/crates/protologic_core).

## Project Structure

The project is split into two Rust "crates".

### protologic_core

`protologic_core` contains the bindings to the game API. `lowlevel` are the direct bindings to the unsafe WASM API. `highlevel` are slightly nicer to use wrappers around the low level bindings. You should never need to edit this project.

### demo_fleet

`demo_fleet` contains a simple demo fleet. `lib.rs` is the root of this project and contains a `tick` function that the game will call every tick:
```rust
#[no_mangle]
pub extern fn tick()
{
    // Every frame
}
```

`state.rs` is the main implementation of the demo fleet. You can delete all of this, or you can use it as an example framework to get started.

## Special Methods

If your code calls the special method `sched_yield()` then execution of your program will be _immediately_ suspended, execution will resume from that point next frame instead of calling `tick()`. This allows you to write simpler programs, for example:

```rust
radar_trigger(); // Trigger radar, results will be available next frame
sched_yield(); // Wait for the next frame.
for i in 0..radar_get_target_count() {
    // Radar contact!
}
```