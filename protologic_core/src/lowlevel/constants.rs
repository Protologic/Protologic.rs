#[link(wasm_import_module = "protologic")]
extern
{
    pub fn const_get_shipradarminangle() -> f32;
    pub fn const_get_shipradarmaxangle() -> f32;

    pub fn const_get_spaceshipthrust() -> f32;
    pub fn const_get_spaceshipfuelconsumption() -> f32;
    pub fn const_get_fueldensity() -> f32;
    pub fn const_get_shipbasemass() -> f32;
    pub fn const_get_shipwheeltorque() -> f32;

    pub fn const_get_turretshellspeed() -> f32;
}