/*
1.4

The objective of each player is to place the opponent’s king ‘under attack’ in such a way that the
opponent has no legal move.

1.4.1

The player who achieves this goal is said to have ‘checkmated’ the opponent’s king and to have won the game.
Leaving one’s own king under attack, exposing one’s own king to attack and also ’capturing’ the opponent’s king
is not allowed.

1.4.2
The opponent whose king has been checkmated has lost the game.
*/
use super::rule::Rule;
use crate::game::game::Game;
use crate::game::game_state::GameState;
use crate::game::moving::MoveError;

pub struct Implementation {}

impl Implementation {
    pub fn new() -> Implementation {
        Implementation {}
    }
}

impl Rule for Implementation {
    fn validate(&self, game: &Game, _: String) -> Result<(), MoveError> {
        match game.state {
            GameState::BlackCheckmate | GameState::WhiteCheckmate => Err(MoveError {
                reason: "Attempt to move after checkmate".to_string(),
            }),
            _ => Ok(()),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cant_move_after_white_checkmate() {
        let game = Game {
            moves: [].to_vec(),
            state: GameState::WhiteCheckmate,
        };
        let actual = (Implementation {}).validate(&game, "".to_string());
        assert_eq!(
            actual.err().unwrap().reason,
            "Attempt to move after checkmate"
        );
    }

    #[test]
    fn cant_move_after_black_checkmate() {
        let game = Game {
            moves: [].to_vec(),
            state: GameState::BlackCheckmate,
        };
        let actual = (Implementation {}).validate(&game, "".to_string());
        assert_eq!(
            actual.err().unwrap().reason,
            "Attempt to move after checkmate"
        );
    }

    #[test]
    fn does_not_prevent_move_after_other_states() {
        let game = Game {
            moves: [].to_vec(),
            state: GameState::WhiteResigned,
        };
        let actual = (Implementation {}).validate(&game, "".to_string());
        assert!(actual.is_ok());
    }

}
