#[link(wasm_import_module = "protologic")]
extern
{
    pub fn const_get_radarminrange() -> f32;
    pub fn const_get_radarmaxrange() -> f32;
    pub fn const_get_radarminangle() -> f32;
    pub fn const_get_radarmaxangle() -> f32;

    pub fn const_get_spaceshipthrust() -> f32;
    pub fn const_get_spaceshipfuelconsumption() -> f32;
    pub fn const_get_fueldensity() -> f32;
    pub fn const_get_shipbasemass() -> f32;
    pub fn const_get_shipwheeltorque() -> f32;

    pub fn const_get_turretshellspeed() -> f32;
}