use std::vec::*;
use super::game_state::*;
use super::moving::*;

pub struct Game {
    pub state: GameState,
    pub moves: Vec<String>
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: GameState::NotStarted,
            moves: vec![],
        }
    }

    pub fn make_move(&self, next_move: String) -> Result<Game, MoveError> {
        match make_move_internal(self, next_move) {
            Ok(game) => Ok(game),
            Err(e) => Err(e)
        }
    }
}