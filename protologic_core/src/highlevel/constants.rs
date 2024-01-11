#![allow(non_upper_case_globals)]

extern crate paste;

macro_rules! define_protologic_const
{
    ($wasm_name:ident, $rust_name:ident, $t:ty, $doc:literal) =>
    {
        #[cfg(not(feature="mock_protologic"))]
        mod $rust_name {
            use crate::constants::paste::paste;

            paste! {
                static mut [<CACHE_ $rust_name>]: Option<$t> = None;
            }

            #[doc=$doc]
            pub fn $rust_name() -> $t
            {
                unsafe
                {
                    if paste! { [<CACHE_ $rust_name>] }.is_none() {
                        paste! { [<CACHE_ $rust_name>] = Some($wasm_name()); }
                    }
                    return paste! { [<CACHE_ $rust_name>] }.unwrap();
                }
            }

            #[cfg(not(test))]
            #[link(wasm_import_module = "protologic")]
            extern { fn $wasm_name() -> $t; }
        }

        #[allow(unused)]
        #[cfg(feature = "mock_protologic")]
        mod $rust_name
        {
            pub fn $rust_name() -> $t
            {
                panic!("extern called in test mode!");
            }
        }

        pub use self::$rust_name::$rust_name;
    }
}

define_protologic_const!(const_get_tickseconds, tick_duration, f32, "Duration of a single tick in seconds.");

define_protologic_const!(const_get_fueldensity, fuel_density, f32, "Mass of a single liter of rocket fuel.");

define_protologic_const!(const_get_spaceshipthrust, ship_engine_thrust, f32, "Thrust produced by the ship engines at maximum throttle.");
define_protologic_const!(const_get_spaceshipfuelconsumption, ship_engine_fuel_consumption, f32, "Amount of fuel (liter/second) consumed by the ship engines at maximum throttle.");
define_protologic_const!(const_get_spaceshipfuelcapacity, ship_engine_fuel_capacity, f32, "Amount of fuel (liters) stored in the ship fuel tanks.");
define_protologic_const!(const_get_shipbasemass, ship_mass, f32, "Total mass of the ship (kilograms) **not** including fuel.");
define_protologic_const!(const_get_shipwheeltorque, ship_wheel_torque, f32, "Torque produced by the ship momentum wheels at maximum.");
define_protologic_const!(const_get_shipradius, ship_radius, f32, "Radius (meters) of the ship. The ship collision geometry is a perfect sphere.");

define_protologic_const!(const_get_shippertickcpufuel, ship_cpu_fuel_per_tick, i64, "Total amount of CPU instructions the CPU can execute every tick. If this amount is exceeded within a single tick future ticks will be skipped to compensate.");
define_protologic_const!(const_get_wasmmemorylimit, ship_cpu_memory_max, i64, "Total memory (bytes) the CPU may consume.");

define_protologic_const!(const_get_turretminelevation, turret_elevation_min, f32, "Minimum elevation of the gun turrets (degrees).");
define_protologic_const!(const_get_turretmaxelevation, turret_elevation_max, f32, "Maximum elevation of the gun turrets (degrees).");
define_protologic_const!(const_get_turretelevationspeed, turret_elevation_speed, f32, "Rotation speed of the turret when changing elevation (degrees/second).");
define_protologic_const!(const_get_turretbearingspeed, turret_bearing_speed, f32, "Rotation speed of the turret when changing bearing (degrees/second).");

define_protologic_const!(const_get_turretminfuse, turret_fuse_min, f32, "Minimum fuse that can be set on a turret (seconds).");
define_protologic_const!(const_get_turretmaxfuse, turret_fuse_max, f32, "Maximum fuse that can be set on a turret (seconds).");
define_protologic_const!(const_get_turretshellspeed, turret_shell_speed, f32, "Movement speed of shells fired from a turret (meters/second).");
define_protologic_const!(const_get_turretrefiretime, turret_refire_time, f32, "Time between firing shots from a turret (seconds).");
define_protologic_const!(const_get_turretreloadtime, turret_reload_time, f32, "Time required to reload a turret (seconds).");
define_protologic_const!(const_get_turretmagazinecapacityarmourpiercing, turret_magazine_capacity_ap, f32, "Total number of shots in an AP magazine.");
define_protologic_const!(const_get_turretmagazinecapacityflak, turret_magazine_capacity_flak, f32, "Total number of shots in a FLAK magazine.");

define_protologic_const!(const_get_flakshelldamage, turret_shell_damage_flak, f32, "Damage inflicted by a flak shell at zero range (damage falls off with distance squared).");
define_protologic_const!(const_get_flakshellrange, turret_shell_damage_range_flak, f32, "Maximum range a flak shell will inflict damage.");
define_protologic_const!(const_get_apshelldamage, turret_shell_damage_ap, f32, "Damage inflicted by a direct hit from an AP shell.");

define_protologic_const!(const_get_shipradarminangle, ship_radar_angle_min, f32, "Minimum angle of ship RADAR.");
define_protologic_const!(const_get_shipradarmaxangle, ship_radar_angle_max, f32, "Maximum anfle of ship RADAR.");

define_protologic_const!(const_get_shipmissilelaunchercount, ship_missile_launcher_count, i32, "Number of missile launchers mounted on the ship.");
define_protologic_const!(const_get_shipmissilelauncherreloadtime, ship_missile_launcher_reload_time, f32, "Number of seconds required to reload a missile launcher.");