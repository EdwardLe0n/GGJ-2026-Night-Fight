use turbo::{serde_json::to_string, *};

use crate::{turbecs, assets};

use assets::online::online_score_board::{ScoreboardElement, NewElement, LongTermBoard};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct OnlineManager {
    
    pub good_one : Vec<ScoreboardElement>,
    pub current_one : Vec<ScoreboardElement>,

}

impl OnlineManager {
    
    pub fn new() -> Self {

        return Self {
            good_one: Vec::with_capacity(10), 
            current_one: Vec::with_capacity(10)
        };

    }

}

impl OnlineManager {

    pub fn update_w_info(&mut self) {

        self.current_one = LongTermBoard::watch("scoreboard")
            .parse()
            .unwrap_or(LongTermBoard::new()).board;

        // Sanity
        // log!("current == {}, good == {}", self.current_one.len(), self.good_one.len());

        if self.good_one.is_empty() {
           self.good_one = self.current_one.clone();
        }
        else {
            if self.current_one.len() != 0 {

                if self.current_one[0].score != 1 {

                    self.good_one = self.current_one.clone();

                }

            }
        }

    }

    pub fn render(&self) {

        let mut some_str = "Leaderboard".to_string();

        some_str.push('\n');
        some_str.push('\n');

        for element in &self.good_one {

            some_str.push_str(&self.name_to_str(element.player_name));

            some_str.push('\t');
            
            let mut temp = element.score;
            let mut count = 10;

            while temp != 0 {

                temp /= 10;
                count -= 1;

            }

            for _ in 0..count {
                some_str.push('0');    
            }

            some_str.push_str(&element.score.to_string());

            some_str.push('\n');

        }

        text_box!(
            &some_str,
            font = "medium",
            w = screen().w(),
            h = screen().h() / 2,
            x = screen().w() as i32 / -2,
            y = screen().h() as i32 / -4,
            align = "center"
        );

    }

    pub fn name_to_str(&self, some_elemnt : (u8, u8, u8)) -> String {

        let mut some_str = "".to_string();

        some_str.push(some_elemnt.0 as char);
        some_str.push(some_elemnt.1 as char);
        some_str.push(some_elemnt.2 as char);

        return some_str;

    }

}