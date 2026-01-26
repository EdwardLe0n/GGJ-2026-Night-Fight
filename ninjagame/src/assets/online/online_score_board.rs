use turbo::*;
use turbo::os::server::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct ScoreboardElement {

    pub score : u32,
    pub player_name : (u8, u8, u8)

}

impl ScoreboardElement {

    pub fn debug() -> Self {

        return Self { 
            score: 1, 
            player_name: ('I' as u8, 'G' as u8, 'B' as u8) 
        };

    }

    pub fn new(some_score : u32, some_name : (u8, u8, u8)) -> Self {

        return Self { score: some_score, player_name: some_name };

    }

}

#[turbo::os::document(program = "scoreboard")]
pub struct LongTermBoard {
    pub board: Vec<ScoreboardElement>,
}

impl LongTermBoard {

    pub fn new() -> Self {

        let mut temp = Vec::with_capacity(10);

        temp.push(ScoreboardElement::debug());
        temp.push(ScoreboardElement::debug());
        temp.push(ScoreboardElement::debug());
        temp.push(ScoreboardElement::debug());
        temp.push(ScoreboardElement::debug());
        temp.push(ScoreboardElement::debug());
        temp.push(ScoreboardElement::debug());
        temp.push(ScoreboardElement::debug());
        temp.push(ScoreboardElement::debug());
        temp.push(ScoreboardElement::debug());

        return Self { board: temp };

    }

}
 
#[turbo::os::command(program = "scoreboard", name = "newElement")]
pub struct NewElement {
    pub some_element : ScoreboardElement
}
impl CommandHandler for NewElement {
    fn run(&mut self, user_id: &str) -> Result<(), std::io::Error> {
        
        // Read existing counter or default to 0
        
        let mut le_board = fs::read("scoreboard")
            .unwrap_or(LongTermBoard::new());

        // See if new max has been found
        
        let len = le_board.board.len();

        for i in 0..len {

            if self.some_element.score >= le_board.board[i].score {

                le_board.board.insert(i, self.some_element.clone());
                
                break;

            }

        }

        if le_board.board.len() > 10 {
            // keeps board length at 10
            le_board.board.pop();
        }

        // Persist updated counter
        fs::write("scoreboard", &le_board)?;

        Ok(())
    }
}