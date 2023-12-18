pub mod actions;
pub mod queries;
pub mod wasi;
pub mod constants;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadarTargetType
{
    Unknown,

    SpaceBattleShip,
    SpaceHulk,
    Missile,
    //NuclearShell,
    //Explosion
    Asteroid,
    //VictoryMarker
    FlakShell,
    APShell,
}

impl From<i32> for RadarTargetType
{
    fn from(item: i32) -> Self
    {
        return match item
        {
            0 => RadarTargetType::SpaceBattleShip,
            1 => RadarTargetType::SpaceHulk,
            2 => RadarTargetType::Missile,
            //3 => RadarTargetType::NuclearShell,
            //4 => RadarTargetType::Explosion,
            5 => RadarTargetType::Asteroid,
            //6 => RadarTargetType::VictoryMarker,
            7 => RadarTargetType::FlakShell,
            8 => RadarTargetType::APShell,
            _ => RadarTargetType::Unknown,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmmoType
{
    Flak,
    ArmourPiercing,

    Unknown
}

impl From<i32> for AmmoType
{
    fn from(item: i32) -> Self
    {
        return match item
        {
            0 => AmmoType::Flak,
            1 => AmmoType::ArmourPiercing,
            _ => AmmoType::Unknown,
        };
    }
}

impl From<u16> for AmmoType
{
    fn from(item: u16) -> Self
    {
        return match item
        {
            0 => AmmoType::Flak,
            1 => AmmoType::ArmourPiercing,
            _ => AmmoType::Unknown,
        };
    }
}

impl Into<i32> for AmmoType
{
    fn into(self) -> i32
    {
        return match self
        {
            AmmoType::Flak => 0,
            AmmoType::ArmourPiercing => 1,
            AmmoType::Unknown => i32::MAX
        };
    }
}