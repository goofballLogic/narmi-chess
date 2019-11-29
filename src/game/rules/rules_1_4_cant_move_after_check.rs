/*
1.4

The objective of each player is to place the opponent’s king ‘under attack’ in such a way that the
opponent has no legal move.
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
            GameState::Started | GameState::NotStarted => Ok(()),
            _ => Err(MoveError {
                reason: "Attempt to move when game is ended".to_string(),
            }),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    fn validate_for_game(configure_expected: impl Fn(&mut Game)) -> Result<(), MoveError> {
        let mut expected = Game {
            moves: [].to_vec(),
            state: GameState::NotStarted,
        };
        configure_expected(&mut expected);
        (Implementation {}).validate(&expected, "".to_string())
    }

    #[test]
    fn move_test() {
        for state in [
            GameState::NotStarted,
            GameState::Started,
            GameState::BlackCheckmate,
            GameState::WhiteCheckmate,
            GameState::Stalemate,
            GameState::BlackResigned,
            GameState::WhiteResigned,
        ]
        .iter()
        {
            let actual = validate_for_game(|game| {
                game.state = *state;
            });
            match *state {
                GameState::NotStarted | GameState::Started => assert!(actual.is_ok()),
                GameState::BlackCheckmate
                | GameState::WhiteCheckmate
                | GameState::Stalemate
                | GameState::WhiteResigned
                | GameState::BlackResigned => {
                    assert!(
                        actual.is_err(),
                        "Failed to return an error for {:?}",
                        *state
                    );
                    assert_eq!(
                        actual.err().unwrap().reason,
                        "Attempt to move when game is ended"
                    );
                }
            }
        }
    }

}
