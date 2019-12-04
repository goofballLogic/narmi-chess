/*
The chessboard is composed of an 8 x 8 grid of 64 equal squares alternately light (the ‘white’ squares)
and dark (the ‘black’ squares).

The chessboard is placed between the players in such a way that the near corner square to the right of
the player is white.
*/
use super::rule::Rule;
use crate::game::game::Game;
use crate::game::moving::MoveError;
use crate::game::notation::decode;

pub struct Implementation {}

impl Implementation {
    pub fn new() -> Implementation {
        Implementation {}
    }
}

impl Rule for Implementation {
    fn validate(&self, _: &Game, next_move: String) -> Result<(), MoveError> {
        match decode(next_move) {
            Ok(_) => Ok(()),
            Err(_) => Err(MoveError {
                reason: "Move is outside the confines of the chess board".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::game::game_state::GameState;

    #[test]
    fn a_move_must_be_valid_within_the_confines_of_the_board() {
        let game = Game {
            moves: [].to_vec(),
            state: GameState::Stalemate,
        };
        let illegal_move = "i9".to_string(); // pawn to i9
        let actual = (Implementation {}).validate(&game, illegal_move);
        assert_eq!(
            actual.err().unwrap().reason,
            "Move is outside the confines of the chess board"
        );
    }

}
