use turbo::{serde_json::to_string, *};

use crate::{turbecs, assets};


#[turbo::serialize]
#[derive(PartialEq)]
pub struct OnlineManager {
    
    

}

impl OnlineManager {
    
    pub fn new() -> Self {

        return Self {
            
        };

    }

}

impl OnlineManager {

    pub fn update_w_info(&mut self) {

        

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