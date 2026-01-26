use std::collections::HashMap;

use turbo::*;
use turbo::os::server::*;

#[turbo::os::document(program = "hostTracker")]
pub struct HostSheet {
    pub board: HashMap<u32, String>,
}

impl HostSheet {

    pub fn new() -> Self {

        let mut temp = HashMap::new();

        temp.insert(0 as u32, "IGB".to_string());

        return Self { board: temp };

    }

    pub fn empty() -> Self {

        let mut temp = HashMap::new();

        return Self { board: temp };

    }

}

// This will be called when a player can update the database with their key

#[turbo::os::command(program = "hostTracker", name = "newCode")]
pub struct NewCode {
    pub some_code : u32
}

impl NewCode {

    pub fn new(some_u32 : u32) -> Self {

        return Self { some_code: some_u32 };

    }

}

impl CommandHandler for NewCode {
    fn run(&mut self, user_id: &str) -> Result<(), std::io::Error> {

        log!("Making new lobby!");
        
        // Read existing list or default
        
        let mut le_board = fs::read("hostTracker")
            .unwrap_or(HostSheet::new());


        // first checks that it doesn't exist in the database

        if !le_board.board.contains_key(&self.some_code) {

            // adds hosts to database
        
            le_board.board.insert(self.some_code, user_id.to_string());

        }
        else {
            log!("couldn't make new room...");
        }

        // logs all hosts

        for element in &le_board.board {
            log!("host {:?}, has key {:?}", element.1, element.0);
        }

        // Persist updated counter
        fs::write("hostTracker", &le_board)?;

        Ok(())
    }
}

#[turbo::serialize]
pub struct GetID;

#[turbo::serialize]
pub struct GiveID {

    pub player_id : String

}

#[turbo::os::channel(program = "hostTracker", name = "getID")]
pub struct GetIDRequest;
impl ChannelHandler for GetIDRequest { 
    type Recv = GetID; // incoming from client
    type Send = GiveID; // outgoing to client
    fn new() -> Self { 
        Self
    } 
    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> {
        Self::send(user_id, GiveID{player_id : user_id.to_string()})
    }
}