use crate::GameState;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ButtonTypes {
    Default,

    ToTitle,
    Misc,
    
    ToHost,
    ToHostWait,
    ToGame,

    ToJoin,

    Test,

    MobileOn,
    MouseOn,

    Number,
    Clear,

    // User made buttons

}

impl ButtonTypes {

    pub fn can_still_interact(&self, _state : &mut GameState) -> bool {

        match self {
            _default => {return false;}
        }

    }

}