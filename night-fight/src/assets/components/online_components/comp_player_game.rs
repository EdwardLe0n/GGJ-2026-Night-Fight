use std::collections::VecDeque;

// Core directories
use turbo::*;

use crate::{GameState, assets, turbecs::{self, managers::scene_manager::Scenes}};

// Necessary imports

use turbecs::entity::Entity;

use assets::online::{game_channel};

use game_channel::{PlayerGameRequest, PlayerGameRequestNotice, PlayerGameRequestEnum};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct PlayerGameComponent {
    
    pub player_reqs : VecDeque<(PlayerGameRequestEnum, u64)>

}

impl PlayerGameComponent {

    pub fn new() -> Self {

        return Self { player_reqs: VecDeque::with_capacity(5) };

    }

}

impl PlayerGameComponent {
    
    pub fn update(&mut self, state : &mut GameState) {

        

    }

    pub fn render(&self) {

        // let mut some_str = "".to_string();

        // match &self.join_state {
        //     JoinState::Done => {

        //         some_str.push_str("Connected!");

        //         some_str.push('\n');
        //         some_str.push('\n');

        //         some_str.push_str("Wait for host...");

        //     }
        //     _default => {

        //         some_str.push_str("Connecting...");

        //     }
        // }

        // text_box!(
        //     &some_str,
        //     font = "large",
        //     width = 200,
        //     height = 100,
        //     x = -100,
        //     y = 0,
        //     color = 0xffffffff,
        //     align = "center"
        // );

    }

}

impl PlayerGameComponent {

    pub fn add_request(&mut self, some_enum : PlayerGameRequestEnum) {

        self.player_reqs.push_back((some_enum, time::now()));

    }

}