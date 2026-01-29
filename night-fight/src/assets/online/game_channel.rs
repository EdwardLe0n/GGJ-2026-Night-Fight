use turbo::*;
use turbo::os::server::*;

#[turbo::serialize]
pub enum PlayerGameRequest {
    Offense,
    Agile,
    Wait,
    Mixed,

    TankMask,
    NimbleMask,
    AggroMask,
    
    Parry
}
