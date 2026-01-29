use turbo::*;

// Core directories

use crate::turbecs;

use crate::GameState;

// Necessary imports

use turbecs::entity::Entity;
use turbecs::component_system;

use component_system::components::comp_butn::ButtonComponent;
use component_system::component::Component;

use component_system::component::ComponentData;

use component_system::component_types::ComponentTypes;

// Click sensitive functions

pub fn on_click (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // First find our own game data

    let num_locat = _ent.find_component_in_state(ComponentTypes::Number, _state);

    if !num_locat.0 {
        log!("couldn't find number");
        return;
    }

    let mut some_u32 = 0;

    if let ComponentData::Number(some_number_comp) = &_state.component_manager.components[num_locat.1].component_data {

        some_u32 = some_number_comp.number;
        
        // Sanity
        // log!("pressed number : {:?}", some_u32);

    }

    // then adjust the other value

    let ent_locat = _state.find_w_component(ComponentTypes::CodeContainer);

    if !ent_locat.0 {
        log!("nuh uh");
        return;
    }

    let code_locat = _state.entity_manager.entities[ent_locat.1].clone().find_component_in_state(ComponentTypes::CodeContainer, _state);

    if !code_locat.0 {
        log!("some error");
        return;
    }

    if let ComponentData::CodeContainer(some_code_comp) = &mut _state.component_manager.components[code_locat.1].component_data {

        some_code_comp.add_new_num(some_u32);

    } 

}