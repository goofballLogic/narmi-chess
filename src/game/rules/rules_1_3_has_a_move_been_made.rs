/*
1.3	A player is said to ‘have the move’ when his opponent’s move has been ‘made’.
*/
use crate::game::moving::MoveError;
use crate::game::game::Game;
use super::rule::Rule;

pub struct Implementation {}

impl Implementation {
    pub fn new() -> Implementation { Implementation {} }
}

impl Rule for Implementation {
    fn validate(&self, _: &Game, _: String) -> Result<(), MoveError> {
        Ok(())
    }
}