use crate::lowlevel::quickstate::get_general_quickstate;

/// Get the world space position of this vehicle
pub fn vehicle_get_position() -> (f32, f32, f32)
{
    return get_general_quickstate().read_vector3(0);
}

/// Get the world space velocity of this vehicle
pub fn vehicle_get_velocity() -> (f32, f32, f32)
{
    return get_general_quickstate().read_vector3(12);
}

/// Get the current mass of this vehicle (hull mass + fuel mass)
pub fn vehicle_get_mass() -> f32
{
    return get_general_quickstate().read_f32(582);
}

/// Get the world space orientation of this vehicle as a quaternion (XYZW)
pub fn vehicle_get_orientation() -> (f32, f32, f32, f32)
{
    return get_general_quickstate().read_quaternion_xyzw(24);
}

/// Get the world space angular velocity of this vehicle
pub fn vehicle_get_angular_velocity() -> (f32, f32, f32)
{
    return get_general_quickstate().read_vector3(40);
}