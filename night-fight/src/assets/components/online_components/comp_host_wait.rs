// Core directories
use turbo::*;

use crate::{GameState, assets, turbecs::{self, managers::scene_manager::Scenes}};

// Necessary imports

use turbecs::entity::Entity;

use assets::online::{game_channel, host_tracker};

use host_tracker::{PlayerConnectNotice, PlayerBeginNotice, ConnectNotice};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct HostWaitComponent {
    
    pub player_id : String,
    pub lobby_code : u32,
    pub player1 : String,
    pub player2 : String,
    pub ready : bool,

}


impl HostWaitComponent {

    pub fn new(some_id : String, some_u32 : u32) -> Self{

        return Self {
            player_id : some_id,
            lobby_code : some_u32,
            player1 : "".to_string(),
            player2 : "".to_string(),
            ready : false
        };

    }

}

impl HostWaitComponent {
    
    pub fn update(&mut self, ent : &mut Entity, state : &mut GameState) {

        self.listen_for_players(state);
        
        if self.ready {
            self.set_up_new_scene(state);
        }

    }

    pub fn render(&self, state : &mut GameState) {

        let mut some_str = "Lobby code: ".to_string();

        some_str.push_str(&self.lobby_code.to_string());

        some_str.push('\n');
        some_str.push('\n');

        some_str.push_str("Player one status:");

        if self.player1 != "" {

            some_str.push_str(" ready");

        }

        some_str.push('\n');

        some_str.push_str("Player two status:");

        if self.player2 != "" {

            some_str.push_str(" ready");

            some_str.push('\n');
            some_str.push('\n');

            some_str.push_str("Ready to duel!");

        }

        text_box!(
            &some_str,
            font = "large",
            width = screen().w()/2,
            height = 100,
            x = -20,
            y = (screen().h() as i32 / 2) * -1 + 20,
            color = 0xffffffff,
            align = "right"
        );

    }

}

impl HostWaitComponent {

    fn listen_for_players(&mut self, state : &mut GameState) {

        if let Some(conn) = PlayerConnectNotice::subscribe("default") {

            // interpreting all pings

            while let Ok(msg) = conn.recv() { 
                
                log!("Got pinged by a player");

                if self.player2 != "" {

                    // sanity
                    log!("full lobby");
                     
                    let _ = conn.send(&ConnectNotice::new_w_all(msg.target_id, false));

                }
                else {

                    if self.player1 == "" {

                        self.player1 = msg.target_id.clone();

                    } else {

                        self.player2 = msg.target_id.clone();

                        // here we assume both players are ready, so now we're ready!

                        self.ready = true;
                         
                    }

                    log!("added new player");

                    let _ = conn.send(&ConnectNotice::new(msg.target_id));

                }

            }

            // If we haven't yet sent a message to the server, request to get our data
            // if !self.sent {

            //     self.sent = true;

            //     let _ = conn.send(&ConnectNotice::new(self.host_id.clone()));

            // }

        }

    }

    fn set_up_new_scene(&mut self, state : &mut GameState) {

        if let Some(conn) = PlayerBeginNotice::subscribe("default") {

            // let p1 = conn.send(&ConnectNotice::new(self.player1.clone()));
            // let p2 = conn.send(&ConnectNotice::new(self.player2.clone()));

            

        }

    }

}