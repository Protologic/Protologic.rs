macro_rules! define_protologic_const
{
    ($wasm_name:ident, $rust_name:ident, $t:ty) =>
    {
        #[link(wasm_import_module = "protologic")]
        extern { fn $wasm_name() -> $t; }

        pub fn $rust_name() -> $t
        {
            return unsafe { $wasm_name() };
        }
    }
}

define_protologic_const!(const_get_fueldensity, fuel_density, f32);

define_protologic_const!(const_get_spaceshipthrust, ship_engine_thrust, f32);
define_protologic_const!(const_get_spaceshipfuelconsumption, ship_engine_fuel_consumption, f32);
define_protologic_const!(const_get_spaceshipfuelcapacity, ship_engine_fuel_capacity, f32);
define_protologic_const!(const_get_shipbasemass, ship_mass, f32);
define_protologic_const!(const_get_shipwheeltorque, ship_wheel_torque, f32);
define_protologic_const!(const_get_shipradius, ship_radius, f32);

define_protologic_const!(const_get_shippertickcpufuel, ship_cpu_fuel_per_tick, i64);
define_protologic_const!(const_get_wasmmemorylimit, ship_cpu_memory_max, i64);

define_protologic_const!(const_get_turretminelevation, turret_elevation_min, f32);
define_protologic_const!(const_get_turretmaxelevation, turret_elevation_max, f32);
define_protologic_const!(const_get_turretelevationspeed, turret_elevation_speed, f32);
define_protologic_const!(const_get_turretbearingspeed, turret_bearing_speed, f32);

define_protologic_const!(const_get_turretminfuse, turret_fuse_min, f32);
define_protologic_const!(const_get_turretmaxfuse, turret_fuse_max, f32);
define_protologic_const!(const_get_turretshellspeed, turret_shell_speed, f32);
define_protologic_const!(const_get_turretrefiretime, turret_refire_time, f32);
define_protologic_const!(const_get_turretreloadtime, turret_reload_time, f32);
define_protologic_const!(const_get_turretmagazinecapacityarmourpiercing, turret_magazine_capacity_ap, f32);
define_protologic_const!(const_get_turretmagazinecapacityflak, turret_magazine_capacity_flak, f32);

define_protologic_const!(const_get_flakshelldamage, turret_shell_damage_flak, f32);
define_protologic_const!(const_get_flakshellrange, turret_shell_damage_range_flak, f32);
define_protologic_const!(const_get_apshelldamage, turret_shell_damage_ap, f32);

define_protologic_const!(const_get_shipradarminangle, ship_radar_angle_min, f32);
define_protologic_const!(const_get_shipradarmaxangle, ship_radar_angle_max, f32);