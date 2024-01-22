use crate::lowlevel::{self, quickstate::get_general_quickstate};

/// Draw a sphere in space, helpful for debugging. The shape will be removed when the returned "handle" is dropped.
pub fn debug_sphere_create(x: f32, y: f32, z: f32, radius: f32, r: f32, g: f32, b: f32) -> DebugShapeHandle
{
    if !is_debug() {
        return DebugShapeHandle { handle: 0 };
    }

    unsafe
    {
        let id = lowlevel::actions::debug_shape_sphere_create(x, y, z, radius, r, g, b);
        return DebugShapeHandle { handle: id };
    }
}

/// Draw a line in space, helpful for debugging. The shape will be removed when the returned "handle" is dropped.
pub fn debug_line_create(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, r: f32, g: f32, b: f32) -> DebugShapeHandle
{
    if !is_debug() {
        return DebugShapeHandle { handle: 0 };
    }

    unsafe
    {
        let id = lowlevel::actions::debug_shape_line_create(x1, y1, z1, x2, y2, z2, r, g, b);
        return DebugShapeHandle { handle: id };
    }
}

/// Pause the replay when this is called.
pub fn debug_pause()
{
    if !is_debug() {
        return;
    }

    unsafe
    {
        lowlevel::actions::debug_pause();
    }
}

/// Pause the replay when this is called.
pub fn debug_log_data(dataset: &str, column: &str, value: f32)
{
    if !is_debug() {
        return;
    }

    unsafe
    {
        lowlevel::actions::debug_log_data(
            dataset.as_ptr(), dataset.len() as i32,
            column.as_ptr(), column.len() as i32,
            value
        );
    }
}

fn is_debug() -> bool
{
    return get_general_quickstate().read_i32(676) != 0;
}

pub struct DebugShapeHandle
{
    handle: i32
}

impl Drop for DebugShapeHandle
{
    fn drop(&mut self)
    {
        unsafe
        {
            lowlevel::actions::debug_shape_destroy(self.handle);
            self.handle = -1;
        }
    }
}