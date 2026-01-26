// Core directories

#[turbo::serialize]
#[derive(PartialEq)]
pub struct NumberComponent {
    
    pub number : u32
}


impl NumberComponent {

    pub fn new(some_u32 : u32) -> Self{

        return Self {
            number : some_u32
        };

    }

}