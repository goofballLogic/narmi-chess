use super::game::*;

#[derive(Debug, PartialEq)]
pub struct MoveError {
    pub reason: String
}

// given a Game (containing moves), validates the next move and returns either the resulting Game or
// a MoveError explaining why the move could not be made
pub fn make_move_internal(game: &Game, next_move: String) -> Result<Game, MoveError> {

    // mot implemented - just always accepts whatever is passed in as the new move and preserves the previous game state

    let mut moves = game.moves.clone();
    moves.push(next_move);
    Ok(Game {
        state: game.state,
        moves: moves
    })
}