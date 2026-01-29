// Core directories

use turbo::*;

use crate::{turbecs, GameState, assets};

// Necessary imports

use turbecs::entity::Entity;
use turbecs::component_system;
use turbecs::managers::scene_manager::Scenes;

use component_system::components::comp_butn::ButtonComponent;

use component_system::component::ComponentData;

use component_system::component_types::ComponentTypes;

use assets::prefabs::online_set_up_prefabs;

// Click sensitive functions

pub fn on_click (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

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

    let mut some_code = 0;

    if let ComponentData::CodeContainer(some_code_comp) = &mut _state.component_manager.components[code_locat.1].component_data {

        if some_code_comp.code_queue.len() == 0 {
            return;
        }

        for num in &some_code_comp.code_queue {

            some_code *= 10;
            some_code += num;

        }

    } 

    _state.scene_manager.load_scene(Scenes::PlayerWait);

    _state.new_entity_w_comp(&mut online_set_up_prefabs::new_player_wait(some_code));

}