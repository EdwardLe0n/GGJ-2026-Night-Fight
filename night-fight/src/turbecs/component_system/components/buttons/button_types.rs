use crate::GameState;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ButtonTypes {
    Default,

    ToTitle,
    Misc,
    
    ToHost,
    ToHostWait,
    ToHostGame,
    ToGame,

    ToJoin,
    ToJoinWait,

    Offense,
    Agile,
    Mixed,
    Idle,

    Tank,
    Aggro,
    Nimble,
    Parry,

    Test,

    MobileOn,
    MouseOn,

    Number,
    Clear,

}

impl ButtonTypes {

    pub fn can_still_interact(&self, _state : &mut GameState) -> bool {

        match self {
            _default => {return false;}
        }

    }

}