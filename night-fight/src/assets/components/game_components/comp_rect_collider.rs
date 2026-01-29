// Core directories

use turbo::*;

use crate::{GameState, turbecs, assets};

use turbecs::entity::Entity;

use turbecs::helpers::transform::Transform;

use turbecs::managers::collision_manager::{ColliderData, ColliderShape, ColliderCase};

use turbecs::component_system;
use component_system::component_types::ComponentTypes;
use component_system::component::ComponentData;

use assets::prefabs::{general_prefabs};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct RectangleColliderComponent {
    pub transform : Transform,
    pub colld_case : ColliderCase
}

impl RectangleColliderComponent {

    pub fn new(some_transform : Transform, some_case : ColliderCase) -> Self {
        return Self{
            transform : some_transform,
            colld_case : some_case
        };
    }

}

impl RectangleColliderComponent {
    
    pub fn on_awake(&mut self, ent : &mut Entity, state : &mut GameState) {

        state.collision_manager.new_collider(
            ColliderData::new(
                ent.locat,
                ColliderShape::Rectangle,
                self.transform.get_width() as u32,
                self.transform.get_height() as u32,
                self.transform.get_x(),
                -self.transform.get_y()
            ), 
            ent.layer
        );

    }

    pub fn on_destroy(&mut self, ent : &mut Entity, state : &mut GameState) {

        state.collision_manager.remove_collider(ent.locat, ent.get_layer());

    }

}

impl RectangleColliderComponent {
    
    pub fn handle_collision(&mut self, ent : &mut Entity, state : &mut GameState, target_ent_locat : usize) {

        log!("handling some collision!");

        match &self.colld_case {
            ColliderCase::Player => {self.handle_player_collision(ent, state, target_ent_locat);}
            ColliderCase::Enemy => {self.handle_enemy_collision(ent, state, target_ent_locat);}
            _default => {}
        }

    }

    fn handle_player_collision(&mut self, ent : &mut Entity, state : &mut GameState, target_ent_locat : usize) {

        

    }

    fn handle_enemy_collision(&mut self, ent : &mut Entity, state : &mut GameState, target_ent_locat : usize) {

        

    }

}