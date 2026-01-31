// Necessary imports

use turbo::*;

use crate::{turbecs, assets, GameState};

use turbecs::managers::scene_manager::Scenes;

use assets::online::host_tracker;

use host_tracker::HostSheet;
use host_tracker::{ConnectNotice, PlayerConnectNotice, PlayerBeginNotice};

#[turbo::serialize]
#[derive(PartialEq)]
pub enum JoinState {

    Checking,
    Ping,
    Listen(f32),
    Done

}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct PlayerWaitComponent {
    pub lobby_code : u32,
    pub host_id : String,
    pub join_state : JoinState,
    pub sent : bool
}

impl PlayerWaitComponent {

    pub fn new(some_u32 : u32) -> Self{

        return Self {
            lobby_code : some_u32,
            host_id : "".to_string(),
            join_state : JoinState::Checking,
            sent : false
        };

    }

}

impl PlayerWaitComponent {
    
    pub fn update(&mut self, state : &mut GameState) {

        match &mut self.join_state {
            JoinState::Checking => {
                self.handle_checking(state);
            },
            JoinState::Ping => {
                self.handle_ping(state);
            },
            JoinState::Listen(dt) => {

                *dt += state.time_manager.delta;
                
                if *dt >= 5.0 {
                    
                    log!("waited too long");
                    
                    state.scene_manager.load_scene(
                        Scenes::PlayerCode
                    );

                    return;
                }

                self.handle_listen(state);
            },
            JoinState::Done => {
                self.handle_done(state);
            },
            _default => {

            }
        }

    }

    pub fn render(&self) {

        let mut some_str = "".to_string();

        match &self.join_state {
            JoinState::Done => {

                some_str.push_str("Connected!");

                some_str.push('\n');
                some_str.push('\n');

                some_str.push_str("Wait for host...");

            }
            _default => {

                some_str.push_str("Connecting...");

            }
        }

        text_box!(
            &some_str,
            font = "large",
            width = 200,
            height = 100,
            x = -100,
            y = 0,
            color = 0xffffffff,
            align = "center"
        );

    }

}

impl PlayerWaitComponent {

    fn handle_checking(&mut self, state : &mut GameState) {

        let db_ref = HostSheet::watch("hostTracker").parse().unwrap_or(HostSheet::empty());

        // If host sheet is empty, ignore
        if db_ref.board == HostSheet::empty().board {
            return;
        }

        if !db_ref.board.contains_key(&self.lobby_code) {

            log!("room doesn't exist");
            state.scene_manager.load_scene(Scenes::PlayerCode);

        }
        else {

            // Sanity
            log!("found room");

            self.host_id = db_ref.board.get(&self.lobby_code).unwrap_or(&"".to_string()).to_string();

            self.join_state = JoinState::Ping;

        }

    }

    fn handle_ping(&mut self, state : &mut GameState) { 

        if let Some(conn) = PlayerConnectNotice::subscribe("default") {

            let _ = conn.send(&ConnectNotice::new(self.host_id.clone()));
            
            self.join_state = JoinState::Listen(0.0);

            return;

        }

    }

    fn handle_listen(&mut self, state : &mut GameState) {

        if let Some(conn) = PlayerConnectNotice::subscribe("default") {

            while let Ok(msg) = conn.recv() { 
                
                if msg.some_bool {
                    log!("we're in!");
                    self.join_state = JoinState::Done;

                    state.online_manager.first_id = msg.target_id;

                }
                else {
                    log!("lobby full");
                    state.scene_manager.load_scene(
                        Scenes::PlayerCode
                    );
                }

                return;

            }

        }

    }

    fn handle_done(&mut self, state : &mut GameState) {

        // log!("at done");

        if let Some(conn) = PlayerBeginNotice::subscribe("default") {

            while let Ok(msg) = conn.recv() { 
                
                state.scene_manager.load_scene(
                    Scenes::PlayerGameButtons
                );

                return;

            }

        }

    }

}