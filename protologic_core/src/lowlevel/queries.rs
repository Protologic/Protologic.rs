use std::{alloc::Layout, sync::OnceLock};

use crate::AmmoType;

static QUICK_STATE: OnceLock<QuickStateBox> = OnceLock::new();

pub fn get_quickstate() -> &'static QuickStateBox
{
    const SIZE : usize = 1024usize;
    return QUICK_STATE.get_or_init(||
    {
        // Allocate a large buffer and tell the engine about it
        let layout = Layout::new::<[u8; SIZE]>();
        let buffer = unsafe { std::alloc::alloc_zeroed(layout) };
        unsafe { sharedmemory_set_readaddress(buffer, SIZE as i32) };

        return QuickStateBox { ptr: buffer };
    });
}

pub struct QuickStateBox
{
    ptr: *mut u8,
}

unsafe impl Sync for QuickStateBox { }
unsafe impl Send for QuickStateBox { }

impl QuickStateBox
{
    fn read_vector3(&self, addr: usize) -> (f32, f32, f32)
    {
        unsafe
        {
            let x = std::ptr::read_unaligned(self.ptr.add(addr + 0) as *const f32);
            let y = std::ptr::read_unaligned(self.ptr.add(addr + 4) as *const f32);
            let z = std::ptr::read_unaligned(self.ptr.add(addr + 8) as *const f32);
            return (x, y, z);
        }
    }

    fn read_quaternion_xyzw(&self, addr: usize) -> (f32, f32, f32, f32)
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

    fn read_i32(&self, addr: usize) -> i32
    {
        unsafe
        {
            return std::ptr::read_unaligned(self.ptr.add(addr) as *const i32);
        }
    }

    fn read_f32(&self, addr: usize) -> f32
    {
        unsafe
        {
            return std::ptr::read_unaligned(self.ptr.add(addr) as *const f32);
        }
    }

    fn read_u16(&self, addr: usize) -> u16
    {
        unsafe
        {
            return std::ptr::read_unaligned(self.ptr.add(addr) as *const u16);
        }
    }

    pub fn read_position(&self) -> (f32, f32, f32)
    {
        return self.read_vector3(0);
    }

    pub fn read_velocity(&self) -> (f32, f32, f32)
    {
        return self.read_vector3(12);
    }

    pub fn read_orientation(&self) -> (f32, f32, f32, f32)
    {
        return self.read_quaternion_xyzw(24);
    }

    pub fn read_angular_velocity(&self) -> (f32, f32, f32)
    {
        return self.read_vector3(40);
    }

    pub fn read_radar_contact_count(&self) -> i32
    {
        return self.read_i32(52);
    }

    pub fn read_gun_bearing(&self, index: i32) -> f32
    {
        if index < 0 || index > 4 {
            panic!("Unknown gun index: {}", index)
        }

        return self.read_f32((56 + index * 4) as usize);
    }

    pub fn read_gun_elevation(&self, index: i32) -> f32
    {
        if index < 0 || index > 4 {
            panic!("Unknown gun index: {}", index)
        }

        return self.read_f32((152 + index * 4) as usize);
    }

    pub fn read_gun_refiretime(&self, index: i32) -> f32
    {
        if index < 0 || index > 4 {
            panic!("Unknown gun index: {}", index)
        }

        return self.read_f32((244 + index * 4) as usize);
    }

    pub fn read_gun_reloadtime(&self, index: i32) -> f32
    {
        if index < 0 || index > 4 {
            panic!("Unknown gun index: {}", index)
        }

        return self.read_f32((336 + index * 4) as usize);
    }

    pub fn read_gun_magazinecapacity(&self, index: i32) -> u16
    {
        if index < 0 || index > 4 {
            panic!("Unknown gun index: {}", index)
        }

        return self.read_u16((428 + index * 2) as usize);
    }

    pub fn read_gun_magazineremaining(&self, index: i32) -> u16
    {
        if index < 0 || index > 4 {
            panic!("Unknown gun index: {}", index)
        }

        return self.read_u16((474 + index * 2) as usize);
    }

    pub fn read_gun_magazinetype(&self, index: i32) -> AmmoType
    {
        if index < 0 || index > 4 {
            panic!("Unknown gun index: {}", index)
        }

        return self.read_u16((520 + index * 2) as usize).into();
    }

    pub fn read_engine_fuelamount(&self) -> f32
    {
        return self.read_f32(566);
    }

    pub fn read_engine_fuelcapacity(&self) -> f32
    {
        return self.read_f32(570);
    }

    pub fn read_engine_throttle(&self) -> f32
    {
        return self.read_f32(574);
    }

    pub fn read_radar_noise(&self) -> f32
    {
        return self.read_f32(578);
    }
}

#[link(wasm_import_module = "protologic")]
extern
{
    /// Set the address that the engine will write data into for quick access
    pub fn sharedmemory_set_readaddress(addr: *mut u8, len: i32);

    /// Get the current amount of fuel available for CPU execution this tick.
    pub fn cpu_get_fuel() -> i64;

    /// Get all info about all radar targets
    /// Returns the number of contacts read into the buffer, never more than `len` parameter.
    pub fn radar_get_contact_list(ptr: *mut RadarGetContactInfo, len: i32) -> i32;
}

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
pub struct QuickState
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