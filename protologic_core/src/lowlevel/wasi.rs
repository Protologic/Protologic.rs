#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern
{
    /// Yield execution for one frame
    pub fn sched_yield() -> i32;
}