mod asyncify;

pub mod actions;
pub mod quickstate;

pub(crate) struct DangerBox<T>
{
    pub value: T,
}

unsafe impl<T> Sync for DangerBox<T> { }
unsafe impl<T> Send for DangerBox<T> { }