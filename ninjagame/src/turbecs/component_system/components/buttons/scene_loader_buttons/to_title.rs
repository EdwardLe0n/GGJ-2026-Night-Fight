use turbo::*;

// Core directories

use crate::{turbecs, assets, GameState};

// Necessary imports

use turbecs::entity::Entity;
use turbecs::component_system;
use turbecs::managers::scene_manager::Scenes;
use component_system::components::comp_butn::ButtonComponent;

use assets::online::host_tracker;
use host_tracker::RemoveCode;

use assets::game_state::online_manager::OnlineManager;

// Click sensitive functions

pub fn on_click (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Loads title scene!

    _state.scene_manager.load_scene(Scenes::Title);

    update_host_db(_state);

}

fn update_host_db(_state : &mut GameState) {

    // Checks if this was a player
    if _state.online_manager.second_id != "" {

        // Do player specific stuff


    }
    else {

        let tmp = RemoveCode::new(_state.online_manager.lobby_code, _state.online_manager.first_id.clone());
        tmp.exec();       

    }

    // Wipes old online data
    _state.online_manager = OnlineManager::new();


}