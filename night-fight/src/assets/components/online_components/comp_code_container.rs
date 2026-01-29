// Core directories

use std::collections::{VecDeque};

use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct CodeContainerComponent {
    
    pub code_queue : VecDeque<u32>

}


impl CodeContainerComponent {

    pub fn new() -> Self{

        return Self {
            code_queue : VecDeque::with_capacity(9)
        };

    }

}

impl CodeContainerComponent {
    
    pub fn render(&self) {
        
        let mut num_to_render = 0;

        for num in &self.code_queue {

            num_to_render *= 10;

            num_to_render += num;

        }

        let mut some_str = "Code: ".to_string();

        if self.code_queue.len() > 0 {
            some_str.push_str(&num_to_render.to_string());
        }

        text_box!(
            &some_str,
            font = "large",
            w = 200,
            h = 50,
            x = -100,
            y = -100,
            align = "center"
        );

    }

}

impl CodeContainerComponent {

    pub fn add_new_num(&mut self, some_u32 : u32) {

        self.code_queue.push_back(some_u32);

        if self.code_queue.len() > 9 {

            self.code_queue.pop_front();
        
        }

    }

    pub fn clear(&mut self) {

        self.code_queue.clear();
        
    }

}