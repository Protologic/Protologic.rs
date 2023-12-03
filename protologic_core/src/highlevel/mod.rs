pub mod actions;
pub mod queries;
pub mod wasi;

#[derive(Eq, PartialEq, Debug)]
pub enum RadarTargetType
{
    Unknown,

    SpaceBattleShip,
    SpaceHulk,
    Missile,
    Shell ,
    Asteroid,
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
            3 => RadarTargetType::Shell,
            5 => RadarTargetType::Asteroid,
            _ => RadarTargetType::Unknown
        };
    }
}