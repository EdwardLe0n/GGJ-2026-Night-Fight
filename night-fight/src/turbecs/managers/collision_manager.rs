use turbo::*;
use crate::{turbecs, GameState};

use turbecs::component_system;
use component_system::component::ComponentData;
use component_system::component_types::ComponentTypes;

const RENDER_COLLISIONS : bool = false;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ColliderShape {
    Rectangle,
}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ColliderCase {
    None,
    Enemy,
    Player
}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct ColliderData {
    ent_locat : usize,
    coll_type : ColliderShape,
    width : u32,
    height : u32,
    x_offset : i32,
    y_offset : i32
}

impl ColliderData {

    pub fn new(some_locat : usize, some_type : ColliderShape, some_width : u32, some_height : u32, some_x : i32, some_y : i32) -> Self {
        return Self {
            ent_locat: some_locat,
            coll_type: some_type,
            width: some_width,
            height: some_height,
            x_offset: some_x,
            y_offset: some_y
        };
    }

}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct CollisionManager {
    pub collision_map : Vec<Vec<(ColliderData)>>,
}

impl CollisionManager {
    
    pub fn new() -> Self {

        return Self { 
            collision_map: Vec::with_capacity(10),  
        };

    }

}

/*
* Lifetime functions
*/

/// Entity lifetime driver for all on_awake functionality

impl CollisionManager {
    
    pub fn new_collider(&mut self, some_collider : ColliderData, some_layer : usize) {

        // Continues to allocate space until

        while self.collision_map.len() <= some_layer {
            
            self.collision_map.push(Vec::with_capacity(10));

        }

        self.collision_map[some_layer].push(some_collider);

    }

    pub fn remove_collider(&mut self, some_ent_locat : usize, some_ent_layer : usize) {

        if some_ent_layer > self.collision_map.len() {
            return;
        }

        let len = self.collision_map[some_ent_layer].len();

        for i in 0..len {

            if self.collision_map[some_ent_layer][i].ent_locat == some_ent_locat {

                self.collision_map[some_ent_layer].remove(i);

                // Sanity
                // log!("found correct collider to destroy! ->  at layer {}", some_ent_layer);

                return;

            }

        }

    }

}

impl GameState {

    pub fn handle_collisions(&mut self) {

        let layers = self.collision_manager.collision_map.len();

        for i in 0..layers {

            let elements = self.collision_manager.collision_map[i].len();

            for j in 0..elements {

                match i {
                    5 => {
                        self.check_layer_for_collisions((i, j), 4);
                    },
                    6 => {
                        self.check_layer_for_collisions((i, j), 3);
                    },
                    _default=> {}
                }

            }

        }

    }

    fn check_layer_for_collisions(&mut self, from : (usize, usize), to_layer : usize) {

        if self.collision_manager.collision_map.len() < to_layer {
            return;
        }

        let elements = self.collision_manager.collision_map[to_layer].len();

        for i in 0..elements {

            // If we're not colliding, ignore

            if !self.check_if_colliding(from, (to_layer, i)) {

                continue;

            }

            // Make a copy of the entity we're from and see if we have a collider
            // For now, only rectangles are hardcoded in 

            let mut from_ent = self.entity_manager.entities[self.collision_manager.collision_map[from.0][from.1].ent_locat].clone();

            let data = from_ent.find_component_in_state(ComponentTypes::RectangleCollider, self);

            if !data.0 {
                continue;
            }

            // Make a copy of the collider component, and handle the collision

            let mut from_component = self.component_manager.components[data.1].clone();

            if let ComponentData::RectangleCollider(some_rect_colld_comp) = &mut from_component.component_data {
                
                some_rect_colld_comp.handle_collision(
                    &mut from_ent,
                    self,
                    self.collision_manager.collision_map[to_layer][i].ent_locat
                );

            }

            // store information back in the state

            let ent_locat = from_ent.locat;

            self.component_manager.components[data.1] = from_component;
            self.entity_manager.entities[ent_locat] = from_ent;

        }

    }

    fn check_if_colliding(&self, from : (usize, usize), to : (usize, usize)) -> bool {

        let a = self.collision_manager.collision_map[from.0][from.1].clone();
        let b = self.collision_manager.collision_map[to.0][to.1].clone();

        let a_x = a.x_offset + self.entity_manager.entities[a.ent_locat].transform.get_x();
        let b_x = b.x_offset + self.entity_manager.entities[b.ent_locat].transform.get_x();

        let a_y = -a.y_offset + self.entity_manager.entities[a.ent_locat].transform.get_y();
        let b_y = -b.y_offset + self.entity_manager.entities[b.ent_locat].transform.get_y();

        if (a_x < b_x + b.width as i32) && (a_x + a.width as i32 > b_x) {

            // Sanity
            // log!("overlapping x!");

            if (a_y < b_y + b.height as i32) && (a_y + a.height as i32 > b_y) {

                // Sanity
                // log!("overlapping: from {}, {} -> to {}, {}", from.0, a.ent_locat, to.0, b.ent_locat);

                return true;

            }

        }

        return false;

    }
    
    pub fn render_colliders(&self) {

        if !RENDER_COLLISIONS {
            return;
        }

        for layer in &self.collision_manager.collision_map {

            for element in layer {

                rect!(
                    w = element.width,
                    h = element.height,
                    x = self.entity_manager.entities[element.ent_locat].transform.get_x() + element.x_offset,
                    y = self.entity_manager.entities[element.ent_locat].transform.get_y() - element.y_offset,
                    color = 0xaa0000a0,
                )

            }

        }

    }

}