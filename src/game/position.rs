use wasm_bindgen::prelude::*;
use super::piece_type::*;
use std::vec::*;


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Position {
    pub rank: u8,
    pub file: u8,
    pub piece_type: PieceType,
    pub is_white: bool
}

// Given an array of moves, calculates an array of positions to be rendered
pub fn calculate_positions(_moves: Vec<String>) -> Vec<Position> {

    // not yet implemented - just returns the initial game layout!
    vec![
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

    ]
    .to_vec()
}