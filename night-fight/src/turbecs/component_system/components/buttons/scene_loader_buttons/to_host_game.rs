use turbo::*;

// Core directories

use crate::{turbecs, assets};

use crate::GameState;

// Necessary imports

use turbecs::entity::Entity;
use turbecs::component_system;
use turbecs::managers::scene_manager::Scenes;

use component_system::components::comp_butn::ButtonComponent;

use component_system::component_types::ComponentTypes;
use component_system::component::ComponentData;

// Click sensitive functions

pub fn on_click (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    let man_ent = _state.find_w_component(ComponentTypes::HostWait);

    if !man_ent.0 {

        log!("couldn't find the manager");
        return;

    }

    let comp_locat = _state.entity_manager.entities[man_ent.1].clone().find_component_in_state(ComponentTypes::HostWait, _state);

    if !comp_locat.0 {

        log!("couldn't find component");
        return;

    }

    if let ComponentData::HostWait(host_wait_comp) = &mut _state.component_manager.components[comp_locat.1].component_data {

        log!("changed it");
        host_wait_comp.send = true;

    }

}