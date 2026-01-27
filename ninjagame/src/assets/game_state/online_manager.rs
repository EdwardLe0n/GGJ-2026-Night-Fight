use turbo::{serde_json::to_string, *};

use crate::{turbecs, assets};

use assets::online;

use online::host_tracker::{ClearHostList, InitCode};


#[turbo::serialize]
#[derive(PartialEq)]
pub struct OnlineManager {
    
    

}

impl OnlineManager {
    
    pub fn new() -> Self {

        // NEEDED, game breaks otherwise

        let start = InitCode::new();
        start.exec();

        return Self {
            
        };

    }

}

impl OnlineManager {

    pub fn update(&mut self) {

        self.handle_reset();

    }

    pub fn render(&self) {

        

    }

    pub fn name_to_str(&self, some_elemnt : (u8, u8, u8)) -> String {

        let mut some_str = "".to_string();

        some_str.push(some_elemnt.0 as char);
        some_str.push(some_elemnt.1 as char);
        some_str.push(some_elemnt.2 as char);

        return some_str;

    }

}

impl OnlineManager {
    
    pub fn handle_reset(&self) {

        let keyboard = keyboard::get();

        if keyboard.key_l().just_pressed() {

            let tmp = ClearHostList::exec(ClearHostList {  });

        }

    }

}