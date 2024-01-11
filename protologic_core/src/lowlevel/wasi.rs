#[cfg(not(test))]
#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern
{
    /// Yield execution for one frame
    pub fn sched_yield() -> i32;
}

/// Yield execution for one frame
#[cfg(test)]
pub unsafe fn sched_yield() -> i32 { panic!("extern called in test mode!") }