// Core directories
use turbo::*;

use crate::{GameState, assets, turbecs::{self, managers::scene_manager::Scenes}};

// Necessary imports

use turbecs::entity::Entity;

use assets::online::game_channel;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct HostWaitComponent {
    
    pub player_id : String,
    pub lobby_code : u32,
    pub player1 : String,
    pub contacted_p1 : bool,
    pub player2 : String,
    pub contacted_p2 : bool,

}


impl HostWaitComponent {

    pub fn new(some_id : String, some_u32 : u32) -> Self{

        return Self {
            player_id : some_id,
            lobby_code : some_u32,
            player1 : "".to_string(),
            contacted_p1 : false,
            player2 : "".to_string(),
            contacted_p2 : false
        };

    }

}

impl HostWaitComponent {
    
    pub fn update(&mut self, ent : &mut Entity, state : &mut GameState) {

        self.listen_for_players();

    }

}

impl HostWaitComponent {

    fn listen_for_players(&mut self) {

    }

}