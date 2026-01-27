// Core directories
use turbo::*;

use crate::{GameState, assets, turbecs::{self, managers::scene_manager::Scenes}};

// Necessary imports

use turbecs::entity::Entity;

use assets::online::host_tracker;

use host_tracker::{GetID, GetIDRequest, HostSheet, NewCode};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct HostCheckComponent {
    
    pub player_id : String,
    pub lobby_code : u32,
    pub sent : bool,
    pub clear : bool,

    pub db_ref : HostSheet,

}


impl HostCheckComponent {

    pub fn new(some_u32 : u32) -> Self{

        return Self {
            player_id : "".to_string(),
            lobby_code : some_u32,
            sent : false,
            clear : false,
            db_ref : HostSheet::empty()
        };

    }

}

impl HostCheckComponent {
    
    pub fn update(&mut self, ent : &mut Entity, state : &mut GameState) {

        if &self.player_id == "" {

            self.get_id();

        }
        else {

            if !self.clear {

                self.check_db(state);

            }

        }

    }

}

impl HostCheckComponent {

    pub fn get_id(&mut self) {

        if let Some(conn) = GetIDRequest::subscribe("default") {

            while let Ok(msg) = conn.recv() { 
                self.player_id = msg.player_id;
                
                log!("Got id!");

                self.sent = false;

                return;

            }

            // If we haven't yet sent a message to the server, request to get our data
            if !self.sent {

                self.sent = true;

                let _ = conn.send(&GetID);

            }

        }

    }

    pub fn check_db(&mut self, state : &mut GameState) {

        // Sanity
        // log!("checking db");

        self.db_ref = HostSheet::watch("hostTracker").parse().unwrap_or(HostSheet::empty());

        // If host sheet is empty, ignore
        if self.db_ref.board == HostSheet::empty().board {
            return;
        }

        if !self.db_ref.board.contains_key(&self.lobby_code) {

            let cmd = NewCode::new(self.lobby_code);
            cmd.exec();

            log!("Storing lobby code!");

            self.clear = true;

        }
        else {

            if self.player_id != (self.db_ref.board.get(&self.lobby_code).unwrap_or(&"".to_string())).to_string() {
                state.scene_manager.load_scene(Scenes::HostCode);
            }

        }

    }

}