mod asyncify;

pub mod actions;
pub mod quickstate;

pub(crate) struct DangerBox<T>
{
    pub value: T,
}

unsafe impl<T> Sync for DangerBox<T> { }
unsafe impl<T> Send for DangerBox<T> { }

#[cfg(feature = "nalgebra")]
impl From<(f32, f32, f32)> for V3Converter
{
    fn from(value: (f32, f32, f32)) -> Self {
        return V3Converter {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

#[cfg(feature = "nalgebra")]
struct V3Converter
{
    x: f32,
    y: f32,
    z: f32,
}

#[cfg(feature = "nalgebra")]
impl From<V3Converter> for nalgebra::Vector3<f32>
{
    fn from(value: V3Converter) -> Self {
        return nalgebra::Vector3::<f32>::new(value.x, value.y, value.z);
    }
}