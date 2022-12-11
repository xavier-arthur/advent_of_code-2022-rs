use super::game::GameInput;
use super::game::GameResult;

use std::default::Default;

#[derive(Debug)]
pub struct RockPaper {
    pub game_score: u32,
    pub game_result: Option<GameResult>,
    mine: Option<GameInput>,
    theirs: Option<GameInput>
    
}

#[allow(dead_code)]
impl RockPaper {
    const POINTS_ROCK    : u8 = 1;
    const POINTS_PAPER   : u8 = 2;
    const POINTS_SCISSORS: u8 = 3;

    const SCORE_WIN : u8 = 6;
    const SCORE_DRAW: u8 = 3;

    pub fn new(&mut self, mine: &str, theirs: &str) {
        self.mine   = Some(Self::parse_input(mine).unwrap());
        self.theirs = Some(Self::parse_input(theirs).unwrap());
        self.game_result = None;
    }

    fn are_inputs_equivalent(&mut self) -> bool {
        if self.mine == Some(GameInput::X) && self.theirs == Some(GameInput::A) {
            return true;
        } else if self.mine == Some(GameInput::Y) && self.theirs == Some(GameInput::B) {
            return true;
        } else if self.mine == Some(GameInput::Z) && self.theirs == Some(GameInput::C) {
            return true;
        }
        false
    }

    pub fn play(&mut self) {
        let mut points_to_sum: u32 = 0;
        let equivalent   = self.are_inputs_equivalent();
        let mine   = self.mine.as_ref().unwrap();
        let theirs = self.theirs.as_ref().unwrap();

        if  equivalent {
            points_to_sum += Self::SCORE_DRAW as u32;
            self.game_result = Some(GameResult::DRAW);
        } else if
            *mine == GameInput::X &&    *theirs == GameInput::C
            || *mine == GameInput::Y && *theirs == GameInput::A 
            || *mine == GameInput::Z && *theirs == GameInput::B {

            self.game_result = Some(GameResult::WIN);
            points_to_sum += Self::SCORE_WIN as u32;
        } else {
            self.game_result = Some(GameResult::LOSE);
        }

        if *mine == GameInput::X {
            points_to_sum += Self::POINTS_ROCK as u32;
        } else if *mine == GameInput::Y {
            points_to_sum += Self::POINTS_PAPER as u32;
        } else {
            points_to_sum += Self::POINTS_SCISSORS as u32;
        }

        self.game_score += points_to_sum;
    }

    pub fn play_rigged(&mut self) {
        let result_target: GameResult = match &self.mine {
            Some(v) if *v == GameInput::Y => GameResult::DRAW,
            Some(v) if *v == GameInput::X => GameResult::LOSE,
            Some(v) if *v == GameInput::Z => GameResult::WIN,
            Some(_) | None => panic!("couldn't parse target result"),
        };

        if result_target == GameResult::DRAW {
            self.draw();
        } else if result_target == GameResult::WIN {
            self.win();
        }  else {
            self.lose();
        }
    }

    fn parse_input(s: &str) -> Result<GameInput, ()> {
        match s {
            "A" => Ok(GameInput::A),
            "B" => Ok(GameInput::B),
            "C" => Ok(GameInput::C),

            "X" => Ok(GameInput::X),
            "Y" => Ok(GameInput::Y),
            "Z" => Ok(GameInput::Z),

            _ => Err(())
        }
    }

    fn win(&mut self) {
        self.game_result = Some(GameResult::WIN);
        self.game_score += Self::SCORE_WIN as u32;
        self.game_score += match &self.theirs {
            Some(v) if *v == GameInput::A => Self::POINTS_PAPER as u32,
            Some(v) if *v == GameInput::B => Self::POINTS_SCISSORS as u32,
            Some(v) if *v == GameInput::C => Self::POINTS_ROCK as u32,
            Some(_) | None => panic!("couldn't parse")
        };
    }

    fn draw(&mut self) {
        self.game_result = Some(GameResult::DRAW);
        self.game_score += Self::SCORE_DRAW as u32;
        self.game_score += match &self.theirs {
            Some(v) if *v == GameInput::A => Self::POINTS_ROCK as u32,
            Some(v) if *v == GameInput::B => Self::POINTS_PAPER as u32,
            Some(v) if *v == GameInput::C => Self::POINTS_SCISSORS as u32,
            Some(_) | None => panic!("couldn't parse")
        };
    }

    fn lose(&mut self) {
        self.game_result = Some(GameResult::LOSE);
        self.game_score += match &self.theirs {
            Some(v) if *v == GameInput::A => Self::POINTS_SCISSORS as u32,
            Some(v) if *v == GameInput::B => Self::POINTS_ROCK as u32,
            Some(v) if *v == GameInput::C => Self::POINTS_PAPER as u32,
            Some(_) | None => panic!("couldn't parse")
        };
    }

}

impl Default for RockPaper {
    fn default() -> Self {
        Self {
            mine: None,
            theirs: None,
            game_result: None,
            game_score: 0
        }
    }
}