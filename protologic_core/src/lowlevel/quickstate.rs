use std::{alloc::Layout, sync::OnceLock};

use lowlevel::DangerBox;

const SIZE : usize = 1024usize;
static GENERAL_QUICK_STATE: OnceLock<QuickStateBox> = OnceLock::new();
static mut GENERAL_QUICK_STATE_DIRTY: DangerBox<bool> = DangerBox { value: true };

pub(crate) fn get_general_quickstate() -> &'static QuickStateBox
{
    // Get a chunk of memory (initialised on first access) to use for quickstate
    let qs = GENERAL_QUICK_STATE.get_or_init(||
    {
        // Allocate a large buffer and tell the engine about it
        let layout = Layout::new::<[u8; SIZE]>();
        let buffer = unsafe { std::alloc::alloc_zeroed(layout) };
        unsafe { read_quickstate(buffer, SIZE as i32) };

        return QuickStateBox { ptr: buffer };
    });

    // If the buffer has been marked as dirty, read it now and mark it as clean
    unsafe
    {
        if GENERAL_QUICK_STATE_DIRTY.value
        {
            crate::lowlevel::quickstate::read_quickstate(qs.ptr, SIZE as i32);
            GENERAL_QUICK_STATE_DIRTY.value = false;
            core::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
        }
    }

    return qs;
}

pub(crate) unsafe fn quickstate_invalidate_cache()
{
    GENERAL_QUICK_STATE_DIRTY.value = true;
}

protologic_define_extern!(pub(crate) fn read_quickstate(addr: *mut u8, bytes: i32));

pub(crate) struct QuickStateBox
{
    pub ptr: *mut u8,
}

unsafe impl Sync for QuickStateBox { }
unsafe impl Send for QuickStateBox { }

impl QuickStateBox
{
    pub fn read_vector3(&self, addr: usize) -> (f32, f32, f32)
    {
        unsafe
        {
            let x = std::ptr::read_unaligned(self.ptr.add(addr + 0) as *const f32);
            let y = std::ptr::read_unaligned(self.ptr.add(addr + 4) as *const f32);
            let z = std::ptr::read_unaligned(self.ptr.add(addr + 8) as *const f32);
            return (x, y, z);
        }
    }

    pub fn read_quaternion_xyzw(&self, addr: usize) -> (f32, f32, f32, f32)
    {
        unsafe
        {
            let x = std::ptr::read_unaligned(self.ptr.add(addr +  0) as *const f32);
            let y = std::ptr::read_unaligned(self.ptr.add(addr +  4) as *const f32);
            let z = std::ptr::read_unaligned(self.ptr.add(addr +  8) as *const f32);
            let w = std::ptr::read_unaligned(self.ptr.add(addr + 12) as *const f32);
            return (x, y, z, w);
        }
    }

    pub fn read_i32(&self, addr: usize) -> i32
    {
        unsafe
        {
            return std::ptr::read_unaligned(self.ptr.add(addr) as *const i32);
        }
    }

    pub fn read_f32(&self, addr: usize) -> f32
    {
        unsafe
        {
            return std::ptr::read_unaligned(self.ptr.add(addr) as *const f32);
        }
    }

    pub fn read_u16(&self, addr: usize) -> u16
    {
        unsafe
        {
            return std::ptr::read_unaligned(self.ptr.add(addr) as *const u16);
        }
    }

    #[allow(dead_code)]
    pub fn read_i64(&self, addr: usize) -> i64
    {
        unsafe
        {
            return std::ptr::read_unaligned(self.ptr.add(addr) as *const i64);
        }
    }

    #[allow(dead_code)]
    pub fn read_u64(&self, addr: usize) -> u64
    {
        unsafe
        {
            return std::ptr::read_unaligned(self.ptr.add(addr) as *const u64);
        }
    }
}

protologic_define_extern!(pub fn cpu_get_fuel() -> i64);
protologic_define_extern!(pub fn radar_get_contact_list2(ptr: *mut crate::radar::RadarGetContactInfo, count: i32, itemSize: i32) -> i32);