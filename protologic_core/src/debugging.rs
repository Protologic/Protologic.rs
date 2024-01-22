use crate::lowlevel;

/// Draw a sphere in space, helpful for debugging. The shape will be removed when the returned "handle" is dropped.
pub fn debug_sphere_create(x: f32, y: f32, z: f32, radius: f32, r: f32, g: f32, b: f32) -> DebugShapeHandle
{
    unsafe
    {
        let id = lowlevel::actions::debug_shape_sphere_create(x, y, z, radius, r, g, b);
        return DebugShapeHandle { handle: id };
    }
}

/// Draw a line in space, helpful for debugging. The shape will be removed when the returned "handle" is dropped.
pub fn debug_line_create(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, r: f32, g: f32, b: f32) -> DebugShapeHandle
{
    unsafe
    {
        let id = lowlevel::actions::debug_shape_line_create(x1, y1, z1, x2, y2, z2, r, g, b);
        return DebugShapeHandle { handle: id };
    }
}

/// Pause the replay when this is called.
pub fn debug_pause()
{
    unsafe
    {
        lowlevel::actions::debug_pause();
    }
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