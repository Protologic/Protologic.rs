protologic_define_extern!(pub(crate) fn engine_set_throttle(throttle: f32));
protologic_define_extern!(pub(crate) fn wheel_set_torque(x: f32, y: f32, z:f32));

protologic_define_extern!(pub(crate) fn radar_set_angle(angle: f32));
protologic_define_extern!(pub(crate) fn radar_set_bearing(bearing: f32));
protologic_define_extern!(pub(crate) fn radar_set_elevation(elevation: f32));
protologic_define_extern!(pub(crate) fn radar_trigger());

protologic_define_extern!(pub(crate) fn gun0_set_bearing(bearing: f32));
protologic_define_extern!(pub(crate) fn gun0_set_elevation(elevation: f32));
protologic_define_extern!(pub(crate) fn gun0_set_fuse(fuse: f32));
protologic_define_extern!(pub(crate) fn gun0_trigger());
protologic_define_extern!(pub(crate) fn gun0_reload(ammo: i32));
protologic_define_extern!(pub(crate) fn gun1_set_bearing(bearing: f32));
protologic_define_extern!(pub(crate) fn gun1_set_elevation(elevation: f32));
protologic_define_extern!(pub(crate) fn gun1_set_fuse(fuse: f32));
protologic_define_extern!(pub(crate) fn gun1_trigger());
protologic_define_extern!(pub(crate) fn gun1_reload(ammo: i32));
protologic_define_extern!(pub(crate) fn gun2_set_bearing(bearing: f32));
protologic_define_extern!(pub(crate) fn gun2_set_elevation(elevation: f32));
protologic_define_extern!(pub(crate) fn gun2_set_fuse(fuse: f32));
protologic_define_extern!(pub(crate) fn gun2_trigger());
protologic_define_extern!(pub(crate) fn gun2_reload(ammo: i32));
protologic_define_extern!(pub(crate) fn gun3_set_bearing(bearing: f32));
protologic_define_extern!(pub(crate) fn gun3_set_elevation(elevation: f32));
protologic_define_extern!(pub(crate) fn gun3_set_fuse(fuse: f32));
protologic_define_extern!(pub(crate) fn gun3_trigger());
protologic_define_extern!(pub(crate) fn gun3_reload(ammo: i32));

protologic_define_extern!(pub(crate) fn debug_shape_sphere_create(x: f32, y: f32, z: f32, radius: f32, r: f32, g: f32, b: f32) -> i32);
protologic_define_extern!(pub(crate) fn debug_shape_line_create(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, r: f32, g: f32, b: f32) -> i32);
protologic_define_extern!(pub(crate) fn debug_shape_destroy(id: i32));
protologic_define_extern!(pub(crate) fn debug_pause());
protologic_define_extern!(pub(crate) fn debug_log_data(namePtr: *const u8, nameLen: i32, colPtr: *const u8, colLen: i32, value: f32));

protologic_define_extern!(pub(crate) fn missilelauncher_trigger(index: i32));
protologic_define_extern!(pub(crate) fn missilelauncher_set_enginetype(index: i32, engine_type: i32));
protologic_define_extern!(pub(crate) fn missilelauncher_set_warheadtype(index: i32, warhead_type: i32));
protologic_define_extern!(pub(crate) fn missilelauncher_set_fuelload(index: i32, fuel_load: f32));
protologic_define_extern!(pub(crate) fn missilelauncher_get_enginetype(index: i32) -> i32);
protologic_define_extern!(pub(crate) fn missilelauncher_get_warheadtype(index: i32) -> i32);
protologic_define_extern!(pub(crate) fn missilelauncher_get_fuelload(index: i32) -> f32);