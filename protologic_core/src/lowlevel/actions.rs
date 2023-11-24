#[link(wasm_import_module = "protologic")]
extern
{
    pub fn ship_self_destruct();

    pub fn engine_set_throttle(throttle: f32);

    pub fn wheel_set_torque(x: f32, y: f32, z:f32);

    pub fn runninglight_set_state(state: i32);

    pub fn radar_set_angle(angle: f32);
    pub fn radar_set_bearing(bearing: f32);
    pub fn radar_set_elevation(elevation: f32);
    pub fn radar_trigger();

    /// Set the bearing gun 0 should begin turning to
    pub fn gun0_set_bearing(bearing: f32);

    /// Set the elevation gun 0 should begin turning to
    pub fn gun0_set_elevation(elevation: f32);

    /// Set the fuse of shells fire from gun 0
    pub fn gun0_set_fuse(fuse: f32);

    /// Fire gun 0 once
    pub fn gun0_trigger();

    /// Set the bearing gun 1 should begin turning to
    pub fn gun1_set_bearing(bearing: f32);

    /// Set the elevation gun 1 should begin turning to
    pub fn gun1_set_elevation(elevation: f32);

    /// Set the fuse of shells fire from gun 1
    pub fn gun1_set_fuse(fuse: f32);

    /// Fire gun 1 once
    pub fn gun1_trigger();

    /// Set the bearing gun 2 should begin turning to
    pub fn gun2_set_bearing(bearing: f32);

    /// Set the elevation gun 2 should begin turning to
    pub fn gun2_set_elevation(elevation: f32);

    /// Set the fuse of shells fire from gun 2
    pub fn gun2_set_fuse(fuse: f32);

    /// Fire gun 2 once
    pub fn gun2_trigger();

    /// Set the bearing gun 3 should begin turning to
    pub fn gun3_set_bearing(bearing: f32);

    /// Set the elevation gun 3 should begin turning to
    pub fn gun3_set_elevation(elevation: f32);

    /// Set the fuse of shells fire from gun 3
    pub fn gun3_set_fuse(fuse: f32);

    /// Fire gun 3 once
    pub fn gun3_trigger();
}