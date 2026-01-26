use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct RunData {
    pub max_health : f32,
    pub curr_health : f32,
    pub player_name : (u8, u8, u8),
    pub enem_count : i32,
    pub enem_max : i32,
    pub enem_defeated : i32,
}

impl RunData {
    
    pub fn new() -> Self {
        return Self{
            max_health : 20.0,
            curr_health : 20.0,
            player_name : ('K' as u8, 'R' as u8, 'P' as u8),
            enem_count : 0,
            enem_max : 3,
            enem_defeated : 0
        };
    }

    pub fn reset(&mut self) {
        self.max_health = 20.0;
        self.curr_health = 20.0;
        self.enem_count = 0;
        self.enem_max = 3;
        self.enem_defeated = 0;
    }

}

impl RunData {
    
    pub fn render(&self, hi : u32, curr : u32) {

        // render back

        let top_left = (-(screen().w() as i32 / 2), -(screen().h() as i32 / 2));
        let height = screen().h() / 8;
        let offset  = 15;

        rect!(
            w = screen().w(),
            h = height,
            x = top_left.0,
            y = top_left.1,
            color = 0x000000ff,
            rotation = 0,
            border_size = 2,
            border_color = 0xffffffff,
            border_radius = 2,
            fixed = false,
        );

        // render name + health text

        let mut player_name = "".to_string();

        player_name.push(self.player_name.0 as char);
        player_name.push(self.player_name.1 as char);
        player_name.push(self.player_name.2 as char);

        player_name.push_str("'s Health");

        text!(
            &player_name,
            x = top_left.0 + offset / 2,
            y = top_left.1 + (height / 8) as i32,
            color = 0xffffffff,
            font = "medium",
        );

        // Render health bar 

        rect!(
            w = screen().w() / 2,
            h = height / 3,
            x = top_left.0 + (screen().w() / 40) as i32,
            y = top_left.1 + (height / 2) as i32,
            color = 0x000000ff,
            rotation = 0,
            border_size = 1,
            border_color = 0xffffffff,
            border_radius = 2,
            fixed = false,
        );

        rect!(
            w = ((screen().w() / 2) as f32 * (self.curr_health / self.max_health)) as i32 - 2,
            h = (height / 3) as i32 - 2,
            x = top_left.0 + (screen().w() / 40) as i32 + 1,
            y = top_left.1 + (height / 2) as i32 + 1,
            color = 0xa00000ff,
            rotation = 0,
            fixed = false,
        );

        // Render hi score

        let mut hi_str = "Hi-Scor:  ".to_string();

        let mut temp = hi;
        let mut count = 10;

        while temp != 0 {

            temp /= 10;
            count -= 1;

        }

        for _ in 0..count {
            hi_str.push('0');    
        }

        hi_str.push_str(&hi.to_string());

        text!(
            &hi_str,
            x = offset,
            y = top_left.1 + (height / 8) as i32,
            color = 0xffffffff,
            font = "medium",
        );

        // render curr score

        let mut curr_str = "Curr-Scr: ".to_string();

        temp = curr;
        count = 10;

        while temp != 0 {

            temp /= 10;
            count -= 1;

        }

        for _ in 0..count {
            curr_str.push('0');    
        }

        curr_str.push_str(&curr.to_string());

        text!(
            &curr_str,
            x = offset,
            y = top_left.1 + (height / 2) as i32,
            color = 0xffffffff,
            font = "medium",
        );

    }

}

impl RunData {
    
    pub fn update_curr(&mut self, new_curr : f32) {

        self.curr_health = new_curr;

        if self.curr_health > self.max_health {

            self.curr_health = self.curr_health.max(self.max_health);

        } 

    }

}