use super::piece_type::*;
use super::notation::*;
use std::vec::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub rank: u8,
    pub file: u8,
    pub piece_type: PieceType,
    pub is_white: bool
}

const INITIAL_POSITIONS: [Position; 32] = [
    Position { rank: 0, file: 0, piece_type: PieceType::Rook, is_white: true },
    Position { rank: 0, file: 1, piece_type: PieceType::Knight, is_white: true },
    Position { rank: 0, file: 2, piece_type: PieceType::Bishop, is_white: true },
    Position { rank: 0, file: 3, piece_type: PieceType::Queen, is_white: true },
    Position { rank: 0, file: 4, piece_type: PieceType::King, is_white: true },
    Position { rank: 0, file: 5, piece_type: PieceType::Bishop, is_white: true },
    Position { rank: 0, file: 6, piece_type: PieceType::Knight, is_white: true },
    Position { rank: 0, file: 7, piece_type: PieceType::Rook, is_white: true },

    Position { rank: 1, file: 0, piece_type: PieceType::Pawn, is_white: true },
    Position { rank: 1, file: 1, piece_type: PieceType::Pawn, is_white: true },
    Position { rank: 1, file: 2, piece_type: PieceType::Pawn, is_white: true },
    Position { rank: 1, file: 3, piece_type: PieceType::Pawn, is_white: true },
    Position { rank: 1, file: 4, piece_type: PieceType::Pawn, is_white: true },
    Position { rank: 1, file: 5, piece_type: PieceType::Pawn, is_white: true },
    Position { rank: 1, file: 6, piece_type: PieceType::Pawn, is_white: true },
    Position { rank: 1, file: 7, piece_type: PieceType::Pawn, is_white: true },

    Position { rank: 6, file: 0, piece_type: PieceType::Pawn, is_white: false },
    Position { rank: 6, file: 1, piece_type: PieceType::Pawn, is_white: false },
    Position { rank: 6, file: 2, piece_type: PieceType::Pawn, is_white: false },
    Position { rank: 6, file: 3, piece_type: PieceType::Pawn, is_white: false },
    Position { rank: 6, file: 4, piece_type: PieceType::Pawn, is_white: false },
    Position { rank: 6, file: 5, piece_type: PieceType::Pawn, is_white: false },
    Position { rank: 6, file: 6, piece_type: PieceType::Pawn, is_white: false },
    Position { rank: 6, file: 7, piece_type: PieceType::Pawn, is_white: false },

    Position { rank: 7, file: 0, piece_type: PieceType::Rook, is_white: false },
    Position { rank: 7, file: 1, piece_type: PieceType::Knight, is_white: false },
    Position { rank: 7, file: 2, piece_type: PieceType::Bishop, is_white: false },
    Position { rank: 7, file: 3, piece_type: PieceType::Queen, is_white: false },
    Position { rank: 7, file: 4, piece_type: PieceType::King, is_white: false },
    Position { rank: 7, file: 5, piece_type: PieceType::Bishop, is_white: false },
    Position { rank: 7, file: 6, piece_type: PieceType::Knight, is_white: false },
    Position { rank: 7, file: 7, piece_type: PieceType::Rook, is_white: true },
];

pub fn calculate_next_positions(prior_positions: Vec<Position>, next_move: &String) -> Vec<Position> {
    println!("{}", next_move);
    prior_positions
}

// Given an array of moves, calculates an array of positions to be rendered
pub fn calculate_positions(moves: Vec<String>) -> Vec<Position> {
    let mut positions = moves.iter().fold(INITIAL_POSITIONS.to_vec(), calculate_next_positions);
    positions.sort();
    positions
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn a_single_move() {
    //     let moves = vec!["e4".to_string()];

    //     let mut expected = INITIAL_POSITIONS.to_vec();
    //     let mut moving_piece = expected
    //         .iter_mut()
    //         .find(|position| (**position).rank == 1 && (**position).file == 4)
    //         .unwrap();
    //     moving_piece.rank = 3;
    //     expected.sort();

    //     let actual = calculate_positions(moves);

    //     assert_eq!(expected, actual);

    // }
}