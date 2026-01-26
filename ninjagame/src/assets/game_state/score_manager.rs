use turbo::*;

use crate::{turbecs};
#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub struct ScoreManager {
    pub current_score : u32,
    pub hi_score : u32
}

impl ScoreManager {

    pub fn new() -> Self {
        
        return Self {
            current_score : 0,
            hi_score : 0
        };

    }

}

impl ScoreManager {
    
    pub fn reset_curr(&mut self) {
        self.current_score = 0;
    }

    pub fn add_to_curr(&mut self, some_points : u32) {

        self.current_score += some_points;

        // Sanity
        // log!("The new score is {}", self.current_score);

        if self.current_score > self.hi_score {
            self.hi_score = self.current_score;
        }

    }

}