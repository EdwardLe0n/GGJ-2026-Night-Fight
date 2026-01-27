use std::collections::HashMap;

use turbo::*;
use turbo::os::server::*;

#[turbo::os::document(program = "hostTracker")]
#[derive(PartialEq)]
pub struct HostSheet {
    pub board: HashMap<u32, String>,
}

impl HostSheet {

    pub fn new() -> Self {

        return Self::empty();

    }

    pub fn empty() -> Self {

        let mut temp = HashMap::new();

        return Self { board: temp };

    }

}

// This will be called when a player can update the database with their key

#[turbo::os::command(program = "hostTracker", name = "initCode")]
pub struct InitCode {
    pub some_code : u32,
    pub some_str : String
}

impl InitCode {

    pub fn new() -> Self {

        return Self { 
            some_code: 2026,
            some_str : "GotBoredGames".to_string()
        };

    }

}

impl CommandHandler for InitCode {
    fn run(&mut self, user_id: &str) -> Result<(), std::io::Error> {

        // log!("Making new lobby!");
        
        // Read existing list or default
        
        let mut le_board = fs::read("hostTracker")
            .unwrap_or(HostSheet::new());


        // first checks that it doesn't exist in the database

        if !le_board.board.contains_key(&self.some_code) {

            // adds hosts to database
        
            le_board.board.insert(self.some_code, self.some_str.to_owned());

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

// This will be called when a host goes back to the main menu, therefore freeing up that key

#[turbo::os::command(program = "hostTracker", name = "removeCode")]
pub struct RemoveCode {
    pub some_code : u32,
    pub some_str : String
}

impl RemoveCode {

    pub fn new(some_u32 : u32, some_str : String) -> Self {

        return Self { 
            some_code: some_u32,
            some_str : some_str
        };

    }

}

impl CommandHandler for RemoveCode {
    fn run(&mut self, user_id: &str) -> Result<(), std::io::Error> {

        // log!("Making new lobby!");
        
        // Read existing list or default
        
        let mut le_board = fs::read("hostTracker")
            .unwrap_or(HostSheet::new());


        // first checks that it doesn't exist in the database

        if le_board.board.contains_key(&self.some_code) {

            if le_board.board.get(&self.some_code).unwrap_or(&"".to_string()) == &self.some_str {
                le_board.board.remove(&self.some_code);
                log!("freed up host key");
            }

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

// This will be called by a client to reset all lobbies

#[turbo::os::command(program = "hostTracker", name = "clearList")]
pub struct ClearHostList {
    
}

impl CommandHandler for ClearHostList {
    fn run(&mut self, user_id: &str) -> Result<(), std::io::Error> {

        if user_id == "63c55578-a270-42a9-9084-c89191a53418" {

            log!("Resetting online system");
        
            // Read existing list or default
            
            let le_board = HostSheet::new();

            // logs all hosts

            for element in &le_board.board {
                log!("host {:?}, has key {:?}", element.1, element.0);
            }

            // Persist updated counter
            fs::write("hostTracker", &le_board)?;

        }

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

#[turbo::serialize]
pub struct ConnectNotice {
    pub target_id : String,
    pub some_bool : bool
}

impl ConnectNotice {
    
    pub fn new(some_str : String) -> Self {
        return Self {
            target_id : some_str,
            some_bool : true
        };
    }

    pub fn new_w_all(some_str : String, some_bool : bool) -> Self {
        return Self {
            target_id : some_str,
            some_bool : some_bool
        };
    }

}

#[turbo::os::channel(program = "hostTracker", name = "playerConnectNotice")]
pub struct PlayerConnectNotice;
impl ChannelHandler for PlayerConnectNotice { 
    type Recv = ConnectNotice; // incoming from client
    type Send = ConnectNotice; // outgoing to client
    fn new() -> Self { 
        Self
    } 
    fn on_data(&mut self, user_id: &str, data: Self::Recv) -> Result<(), std::io::Error> {
        Self::send(&data.target_id, ConnectNotice::new_w_all(user_id.to_owned(), data.some_bool))
    }
}