#[link(wasm_import_module = "protologic")]
extern
{
    pub fn const_get_fueldensity() -> f32;

    pub fn const_get_spaceshipthrust() -> f32;
    pub fn const_get_spaceshipfuelconsumption() -> f32;

    pub fn const_get_missilethrust() -> f32;
    pub fn const_get_missilefuelconsumption() -> f32;

    pub fn const_get_turretminelevation() -> f32;
    pub fn const_get_turretmaxelevation() -> f32;
    pub fn const_get_turretelevationspeed() -> f32;
    pub fn const_get_turretbearingspeed() -> f32;
    pub fn const_get_turretminfuse() -> f32;
    pub fn const_get_turretmaxfuse() -> f32;
    pub fn const_get_turretshellspeed() -> f32;
    pub fn const_get_turretrefiretime() -> f32;

    pub fn const_get_missileradarminrange() -> f32;
    pub fn const_get_missileradarmaxrange() -> f32;
    pub fn const_get_missileradarminangle() -> f32;
    pub fn const_get_missileradarmaxangle() -> f32;

    pub fn const_get_shipradarminangle() -> f32;
    pub fn const_get_shipradarmaxangle() -> f32;

    pub fn const_get_missilerefiretime() -> f32;

    pub fn const_get_shipbasemass() -> f32;

    pub fn const_get_missilebasemass() -> f32;
    pub fn const_get_missilewheeltorque() -> f32;

    pub fn const_get_shipwheeltorque() -> f32;

    pub fn const_get_shipradius() -> f32;

    pub fn const_get_missileradius() -> f32;

    pub fn const_get_shipmaxcpufuel() -> i64;
    pub fn const_get_shipmincpufuel() -> i64;
    
    pub fn const_get_shippertickcpufuel() -> i64;
}