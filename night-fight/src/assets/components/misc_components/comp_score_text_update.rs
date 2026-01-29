// Core directories

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::entity::Entity;
use turbecs::component_system;

use component_system::{component::ComponentData, component_types::ComponentTypes};


#[turbo::serialize]
#[derive(PartialEq)]
pub struct ScoreTextUpdateComponent {
    
}

impl ScoreTextUpdateComponent {

    pub fn new() -> Self {
        return Self {};
    }

}

impl ScoreTextUpdateComponent {
    
    pub fn on_awake(&mut self, ent : &mut Entity, state : &mut GameState) {
    
        let data = ent.find_component_in_state(ComponentTypes::TextBox, state);

        if !data.0 {
            return;
        }

        let mut text_box = state.component_manager.components[data.1].clone();

        if let ComponentData::TextBox(some_text_box) = &mut text_box.component_data {
            
            let mut ref_string = some_text_box.text.clone();

            ref_string.push_str(&state.online_manager.name_to_str(state.run_data.player_name));

            ref_string.push_str(" !\n\nYou got a score of ");
            
            let mut temp = state.score_manager.current_score;
            let mut count = 10;

            while temp != 0 {

                temp /= 10;
                count -= 1;

            }

            for _ in 0..count {
                ref_string.push('0');    
            }

            ref_string.push_str(&(state.score_manager.current_score.to_string()));

            some_text_box.replace_str(ref_string);

        }

        state.component_manager.components[data.1] = text_box;
        
    }

}