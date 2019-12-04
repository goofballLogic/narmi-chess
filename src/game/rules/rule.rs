use crate::game::moving::MoveError;
use crate::game::game::Game;

pub trait Rule {
    fn validate(&self, game: &Game, next_move: String) -> Result<(), MoveError> {
        Ok(())
    }
}