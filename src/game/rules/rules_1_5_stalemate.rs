/*
1.5

If the position is such that neither player can possibly checkmate the opponent’s king, the game is drawn (see Article 5.2.2).
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
                reason: "Attempt to move after stalemate".to_string(),
            }),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn move_test() {
        let game = Game {
            moves: [].to_vec(),
            state: GameState::Stalemate,
        };
        let actual = (Implementation {}).validate(&game, "".to_string());
        assert_eq!(
            actual.err().unwrap().reason,
            "Attempt to move after stalemate"
        );
    }

}
