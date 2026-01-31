// Core directories
use turbo::*;

use crate::{GameState, assets, turbecs::{self, managers::scene_manager::Scenes}};

// Necessary imports

use turbecs::entity::Entity;

use assets::online::{game_channel};

use game_channel::{PlayerGameRequest, PlayerGameRequestNotice, PlayerGameRequestEnum};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct HostGameComponent {
    
    pub p_locat_1 : String,
    pub p_locat_2 : String,
    pub game_done : bool,

}

impl HostGameComponent {

    pub fn new(some_id_1 : String, some_id_2 : String) -> Self {

        return Self {
            p_locat_1 : some_id_1,
            p_locat_2 : some_id_2,
            game_done : false
        };

    }

}