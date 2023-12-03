#[link(wasm_import_module = "protologic")]
extern
{
    /// Get the current amount of fuel available for CPU execution this tick.
    pub fn cpu_get_fuel() -> i64;

    /// Get the X element of thecurrent world space position of this ship
    pub fn ship_get_position_x() -> f32;

    /// Get the Y element of the current world space position of this ship
    pub fn ship_get_position_y() -> f32;

    /// Get the Z element of the current world space position of this ship
    pub fn ship_get_position_z() -> f32;

    /// Get the XYZ world space position into the given destination in memory (3 x f32)
    pub fn ship_get_position_ptr(dst: *mut f32);

    /// Get the X element of the current world space velocity of this ship
    pub fn ship_get_velocity_x() -> f32;

    /// Get the Y element of the  current world space velocity of this ship
    pub fn ship_get_velocity_y() -> f32;

    /// Get the Z element of the  current world space velocity of this ship
    pub fn ship_get_velocity_z() -> f32;

    /// Get the XYZ world space velocity into the given destination in memory (3 x f32)
    pub fn ship_get_velocity_ptr(dst: *mut f32);

    /// Get the W element of the orientation quaternion of this ship
    pub fn ship_get_orientation_w() -> f32;

    /// Get the X element of the orientation quaternion of this ship
    pub fn ship_get_orientation_x() -> f32;

    /// Get the Y element of the orientation quaternion of this ship
    pub fn ship_get_orientation_y() -> f32;

    /// Get the Z element of the orientation quaternion of this ship
    pub fn ship_get_orientation_z() -> f32;

    /// Get the XYZW orientation of this ship (4 x f32)
    pub fn ship_get_orientation_ptr(dst: *mut f32);

    /// Get the X element of the angular_velocity of this ship
    pub fn ship_get_angularvelocity_x() -> f32;

    /// Get the Y element of the angular_velocity of this ship
    pub fn ship_get_angularvelocity_y() -> f32;

    /// Get the Z element of the angular_velocity of this ship
    pub fn ship_get_angularvelocity_z() -> f32;

    /// Get the XYZ angular_velocity of this ship (3 x f32)
    pub fn ship_get_angularvelocity_ptr(dst: *mut f32);

    /// Get the current amount of fuel in the tanks for the rocket engines.
    pub fn engine_get_fuel_amount() -> f32;

    /// Get the maximum amount of fuel in the tanks for the rocket engines.
    pub fn engine_get_fuel_capacity() -> f32;

    /// Get the current engine throttle (0 to 1). This may _not_ be the throttle value set last frame, if running low on fuel.
    pub fn engine_get_throttle() -> f32;

    /// Get the number of targets detected by the radar last time it was triggered
    pub fn radar_get_target_count() -> i32;

    /// Get the the distance to the target with the given index
    pub fn radar_get_target_distance(index: i32) -> f32;

    /// Get the the type of the target with the given index
    pub fn radar_get_target_type(index: i32) -> i32;

    /// Get the the ID of the target with the given index
    pub fn radar_get_target_id(index: i32) -> i64;

    /// Get all info about the target with the given index
    pub fn radar_get_target_info(index: i32, ptr: *mut RadarGetTargetInfo);

    /// Get all info about all radar targets
    pub fn radar_get_target_list(ptr: *mut RadarGetTargetInfo, len: i32) -> i32;

    /// Get the current bearing of gun 0
    pub fn gun0_get_bearing() -> f32;

    /// Get the current elevation of gun 0
    pub fn gun0_get_elevation() -> f32;

    /// Get the number of seconds before gun 0 can fire again
    pub fn gun0_get_refiretime() -> f32;

    /// Get the current bearing of gun 1
    pub fn gun1_get_bearing() -> f32;

    /// Get the current elevation of gun 1
    pub fn gun1_get_elevation() -> f32;

    /// Get the number of seconds before gun 1 can fire again
    pub fn gun1_get_refiretime() -> f32;

    /// Get the current bearing of gun 2
    pub fn gun2_get_bearing() -> f32;

    /// Get the current elevation of gun 2
    pub fn gun2_get_elevation() -> f32;

    /// Get the number of seconds before gun 2 can fire again
    pub fn gun2_get_refiretime() -> f32;

    /// Get the current bearing of gun 3
    pub fn gun3_get_bearing() -> f32;

    /// Get the current elevation of gun 3
    pub fn gun3_get_elevation() -> f32;

    /// Get the number of seconds before gun 3 can fire again
    pub fn gun3_get_refiretime() -> f32;
}

#[repr(C)]
pub struct RadarGetTargetInfo
{
    pub id: i64,
    target_type: i32,
    pub distance: f32
}

impl RadarGetTargetInfo
{
    pub(crate) fn default() -> RadarGetTargetInfo
    {
        return RadarGetTargetInfo {
            id: -1,
            target_type: -1,
            distance: -1f32
        };
    }

    pub fn get_target_type(&self) -> crate::RadarTargetType
    {
        return self.target_type.into();
    }
}