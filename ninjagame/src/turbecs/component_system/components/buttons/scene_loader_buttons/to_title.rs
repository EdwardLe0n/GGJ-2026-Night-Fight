use turbo::*;

// Core directories

use crate::{turbecs, assets, GameState};

// Necessary imports

use turbecs::entity::Entity;
use turbecs::component_system;
use turbecs::managers::scene_manager::Scenes;

use component_system::component_types::ComponentTypes;
use component_system::component::ComponentData;
use component_system::components::comp_butn::ButtonComponent;

use assets::online::host_tracker;
use host_tracker::RemoveCode;

// Click sensitive functions

pub fn on_click (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Loads title scene!

    _state.scene_manager.load_scene(Scenes::Title);

    update_host_db(_state);

}

fn update_host_db(_state : &mut GameState) {
    
    // will let the server know that the host has left

    let ent_locat = _state.find_w_component(ComponentTypes::HostCheck);

    if !ent_locat.0 {
        // Sanity
        // log!("nuh uh");
        return;
    }

    let code_locat = _state.entity_manager.entities[ent_locat.1].clone().find_component_in_state(ComponentTypes::HostCheck, _state);

    if !code_locat.0 {
        // Sanity
        // log!("some error");
        return;
    }

    if let ComponentData::HostCheck(some_host_data) = &mut _state.component_manager.components[code_locat.1].component_data {

        let tmp = RemoveCode::new(some_host_data.lobby_code, some_host_data.player_id.clone());
        tmp.exec();

    } 

}