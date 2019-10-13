/*
1.2	The player with the light-coloured pieces (White) makes the first move,
then the players move alternately, with the player with the dark-coloured pieces (Black) making the next move.
*/

use crate::game::game::*;
use crate::game::moving::*;

pub fn validate(game: &Game, next_move: String) -> Result<(), MoveError> {
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    struct Setup {
        game: Game
    }

    impl Setup {
        pub fn new() -> Setup {
            let game = Game::new();
            Setup {
                game: game
            }
        }
    }

    #[test]
    fn white_can_go_first() {
        let setup = Setup::new();
        let actual = validate(&setup.game, "e4".to_string());
        assert_eq!(actual.ok(), Some(()));
    }

}
