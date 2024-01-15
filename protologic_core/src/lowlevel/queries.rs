use std::{alloc::Layout, sync::OnceLock};

static GENERAL_QUICK_STATE: OnceLock<QuickStateBox> = OnceLock::new();

pub(crate) fn get_general_quickstate() -> &'static QuickStateBox
{
    const SIZE : usize = 1024usize;
    return GENERAL_QUICK_STATE.get_or_init(||
    {
        // Allocate a large buffer and tell the engine about it
        let layout = Layout::new::<[u8; SIZE]>();
        let buffer = unsafe { std::alloc::alloc_zeroed(layout) };
        unsafe { sharedmemory_set_readaddress(buffer, SIZE as i32) };

        return QuickStateBox { ptr: buffer };
    });
}

pub(crate) unsafe fn get_quickstate_buffer_pointer() -> *mut u8
{
    return get_general_quickstate().ptr;
}

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

    pub fn read_u64(&self, addr: usize) -> u64
    {
        unsafe
        {
            return std::ptr::read_unaligned(self.ptr.add(addr) as *const u64);
        }
    }
}

protologic_define_extern!(pub fn sharedmemory_set_readaddress(addr: *mut u8, len: i32));
protologic_define_extern!(pub fn cpu_get_fuel() -> i64);
protologic_define_extern!(pub fn radar_get_contact_list(ptr: *mut super::RadarGetContactInfo, len: i32) -> i32);

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct RadarGetContactInfo
{
    pub id: i64,
    target_type: i32,
    pub signal_strength: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl RadarGetContactInfo
{
    pub fn get_target_type(&self) -> crate::RadarTargetType
    {
        return self.target_type.into();
    }
}

#[repr(C)]
pub(crate) struct QuickState
{
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,

    pub vel_x: f32,
    pub vel_y: f32,
    pub vel_z: f32,

    pub orientation_x: f32,
    pub orientation_y: f32,
    pub orientation_z: f32,
    pub orientation_w: f32,

    pub angular_vel_x: f32,
    pub angular_vel_y: f32,
    pub angular_vel_z: f32,
}

unsafe impl Send for QuickState { }
unsafe impl Sync for QuickState { }

impl Default for QuickState
{
    fn default() -> Self
    {
        Self
        {
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            vel_x: 0.0,
            vel_y: 0.0,
            vel_z: 0.0,
            orientation_x: 0.0,
            orientation_y: 0.0,
            orientation_z: 0.0,
            orientation_w: 0.0,
            angular_vel_x: 0.0,
            angular_vel_y: 0.0,
            angular_vel_z: 0.0,
        }
    }
}