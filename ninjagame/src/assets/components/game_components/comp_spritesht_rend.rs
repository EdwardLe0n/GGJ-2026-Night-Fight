// Core directories

use turbo::*;

use crate::{turbecs, GameState, assets};

use turbecs::helpers::transform::Transform;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct SpriteSheetRendererComponent {
    pub sprite_locat : String,
    pub transform : Transform,
    pub elapsed : f32,
    pub anim_speed : f32,
    pub anim_len : u32,
    pub x_locat : u32,
    pub y_locat :u32 
}

impl SpriteSheetRendererComponent {

    pub fn new(some_str : String, some_transform : Transform, some_speed : f32, some_len : u32, some_x : u32, some_y : u32) -> Self {
        return Self {
            sprite_locat: some_str,
            transform: some_transform,
            elapsed: 0.0,
            anim_speed: some_speed,
            anim_len : some_len,
            x_locat: some_x,
            y_locat: some_y
        };
    }

}

impl SpriteSheetRendererComponent {

    pub fn update(&mut self, state : &mut GameState) {

        self.update_time(state);

    }

    pub fn render(&self, transform : Transform) {

        sprite!(
            &self.sprite_locat,
            x = transform.get_x() + self.transform.get_x(),
            y = transform.get_y() + self.transform.get_y(),
            w = self.transform.get_width(),
            h = self.transform.get_height(),
            tx = -((self.elapsed) as i32) * self.transform.get_width() - (self.x_locat as i32),
            ty = -(self.y_locat as i32),
            cover = false,
            fixed = false
        );

    }
        
}

impl SpriteSheetRendererComponent {
    
    pub fn update_time(&mut self, state : &mut GameState) {

        self.elapsed += state.time_manager.delta * self.anim_speed;

        let len = self.anim_len as f32;

        if self.elapsed >= len {
            self.elapsed -= len;
        }

    }

    pub fn update_animation(&mut self) {

        

    }

    pub fn update_speed(&mut self) {


    }

}