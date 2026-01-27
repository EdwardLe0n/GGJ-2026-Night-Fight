use turbo::*;

use std::collections::VecDeque;

use crate::{turbecs};
use turbecs::entity::Entity;
use turbecs::component_system::component::Component;

use crate::assets;

use assets::prefabs::{general_prefabs, online_set_up_prefabs};

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub struct SceneManager {
    pub active_scene : Scenes,
    pub is_loaded : bool
}

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub enum Scenes {
    DevCards,
    Title,
    Settings,

    HostCode,
    HostWait,
    HostGame,

    PlayerCode,
    PlayerWait,
    PlayerGameButtons,

    Misc
}

impl SceneManager {

    pub fn new() -> Self {

        return Self{active_scene : Scenes::DevCards, is_loaded : false};

    }

    pub fn load_scene(&mut self, _some_scene : Scenes) {

        self.active_scene = _some_scene;
        self.is_loaded = false;

    }
}

pub fn make_scene (some_scene : Scenes) ->  VecDeque<(Entity, VecDeque<Component>)>{

    match some_scene {
        Scenes::DevCards => {return make_dev_card_scene();},
        Scenes::Title => {return make_title_scene()},
        Scenes::Misc => {return make_misc_scene()},

        Scenes::HostCode => {return make_host_code_scene()},
        Scenes::HostWait => {return make_host_wait_scene()},

        Scenes::PlayerCode => {return make_player_code_scene()},

        _default => {
            return VecDeque::new();
        }
    }

}

pub fn make_dev_card_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_turbecs_card());

    return ent_vec;

}

pub fn make_title_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_title());

    ent_vec.push_back(general_prefabs::new_to_host());
    ent_vec.push_back(general_prefabs::new_to_join());

    return ent_vec;

}

pub fn make_misc_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_to_title());

    return ent_vec;

}

pub fn make_host_code_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(online_set_up_prefabs::new_code_container());

    ent_vec.push_back(online_set_up_prefabs::new_number(0));
    ent_vec.push_back(online_set_up_prefabs::new_number(1));
    ent_vec.push_back(online_set_up_prefabs::new_number(2));
    ent_vec.push_back(online_set_up_prefabs::new_number(3));
    ent_vec.push_back(online_set_up_prefabs::new_number(4));
    ent_vec.push_back(online_set_up_prefabs::new_number(5));
    ent_vec.push_back(online_set_up_prefabs::new_number(6));
    ent_vec.push_back(online_set_up_prefabs::new_number(7));
    ent_vec.push_back(online_set_up_prefabs::new_number(8));
    ent_vec.push_back(online_set_up_prefabs::new_number(9));

    ent_vec.push_back(online_set_up_prefabs::new_clear());
    ent_vec.push_back(online_set_up_prefabs::new_to_host_wait());

    ent_vec.push_back(general_prefabs::new_to_title());

    return ent_vec;

}

pub fn make_host_wait_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_to_title());

    return ent_vec;

}

pub fn make_player_code_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(online_set_up_prefabs::new_code_container());

    ent_vec.push_back(online_set_up_prefabs::new_number(0));
    ent_vec.push_back(online_set_up_prefabs::new_number(1));
    ent_vec.push_back(online_set_up_prefabs::new_number(2));
    ent_vec.push_back(online_set_up_prefabs::new_number(3));
    ent_vec.push_back(online_set_up_prefabs::new_number(4));
    ent_vec.push_back(online_set_up_prefabs::new_number(5));
    ent_vec.push_back(online_set_up_prefabs::new_number(6));
    ent_vec.push_back(online_set_up_prefabs::new_number(7));
    ent_vec.push_back(online_set_up_prefabs::new_number(8));
    ent_vec.push_back(online_set_up_prefabs::new_number(9));

    ent_vec.push_back(online_set_up_prefabs::new_clear());
    ent_vec.push_back(online_set_up_prefabs::new_to_player_wait());

    ent_vec.push_back(general_prefabs::new_to_title());

    return ent_vec;

}