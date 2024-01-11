const MIN: f32 = -10000f32;
const MAX: f32 = 10000f32;
const RANGE: f32 = MAX - MIN;

// Pack 3 floats into a single i64 so it can be sent over the radio. This sends 
// each value as a u16, representing how far between MIN and MAX the value is.
pub fn pack_message(pos: (f32, f32, f32)) -> u64
{
    let x = pack_channel(pos.0) as u64;
    let y = pack_channel(pos.1) as u64;
    let z = pack_channel(pos.2) as u64;

    return x << 00
         | y << 16
         | z << 32;

    fn pack_channel(value: f32) -> u16
    {
        return ((u16::MAX as f32) * (value- MIN) / RANGE).floor() as u16;
    }
}

pub fn unpack_message(message: u64) -> (f32, f32, f32)
{
    let x = (message >> 00) & 0xFFFF;
    let y = (message >> 16) & 0xFFFF;
    let z = (message >> 32) & 0xFFFF;

    return (
        unpack_channel(x as u16),
        unpack_channel(y as u16),
        unpack_channel(z as u16),
    );

    fn unpack_channel(value: u16) -> f32
    {
        return ((value as f32) / (u16::MAX as f32)) * RANGE + MIN;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn round_trip()
    {
        let actual = (100.0, 190.0, -320.0);

        let msg = pack_message(actual);
        let pos = unpack_message(msg);
        
        let dist = f32::sqrt(f32::powi(actual.0 - pos.0, 2) + f32::powi(actual.1 - pos.1, 2) + f32::powi(actual.2 - pos.2, 2));
        assert!(dist < 1f32);
    }
}