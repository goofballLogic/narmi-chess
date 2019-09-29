use wasm_bindgen::prelude::*;
use super::position;
use super::game_state;
use super::piece_type;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    positions: Vec<position::Position>,
    is_white_move: bool,
    state: game_state::GameState,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            is_white_move: true,
            state: game_state::GameState::NotStarted,
            positions: [
                position::Position { rank: 0, file: 0, piece_type: piece_type::PieceType::Rook, is_white: true },
                position::Position { rank: 0, file: 1, piece_type: piece_type::PieceType::Knight, is_white: true },
                position::Position { rank: 0, file: 2, piece_type: piece_type::PieceType::Bishop, is_white: true },
                position::Position { rank: 0, file: 3, piece_type: piece_type::PieceType::Queen, is_white: true },
                position::Position { rank: 0, file: 4, piece_type: piece_type::PieceType::King, is_white: true },
                position::Position { rank: 0, file: 5, piece_type: piece_type::PieceType::Bishop, is_white: true },
                position::Position { rank: 0, file: 6, piece_type: piece_type::PieceType::Knight, is_white: true },
                position::Position { rank: 0, file: 7, piece_type: piece_type::PieceType::Rook, is_white: true },

                position::Position { rank: 1, file: 0, piece_type: piece_type::PieceType::Pawn, is_white: true },
                position::Position { rank: 1, file: 1, piece_type: piece_type::PieceType::Pawn, is_white: true },
                position::Position { rank: 1, file: 2, piece_type: piece_type::PieceType::Pawn, is_white: true },
                position::Position { rank: 1, file: 3, piece_type: piece_type::PieceType::Pawn, is_white: true },
                position::Position { rank: 1, file: 4, piece_type: piece_type::PieceType::Pawn, is_white: true },
                position::Position { rank: 1, file: 5, piece_type: piece_type::PieceType::Pawn, is_white: true },
                position::Position { rank: 1, file: 6, piece_type: piece_type::PieceType::Pawn, is_white: true },
                position::Position { rank: 1, file: 7, piece_type: piece_type::PieceType::Pawn, is_white: true },

                position::Position { rank: 6, file: 0, piece_type: piece_type::PieceType::Pawn, is_white: false },
                position::Position { rank: 6, file: 1, piece_type: piece_type::PieceType::Pawn, is_white: false },
                position::Position { rank: 6, file: 2, piece_type: piece_type::PieceType::Pawn, is_white: false },
                position::Position { rank: 6, file: 3, piece_type: piece_type::PieceType::Pawn, is_white: false },
                position::Position { rank: 6, file: 4, piece_type: piece_type::PieceType::Pawn, is_white: false },
                position::Position { rank: 6, file: 5, piece_type: piece_type::PieceType::Pawn, is_white: false },
                position::Position { rank: 6, file: 6, piece_type: piece_type::PieceType::Pawn, is_white: false },
                position::Position { rank: 6, file: 7, piece_type: piece_type::PieceType::Pawn, is_white: false },

                position::Position { rank: 7, file: 0, piece_type: piece_type::PieceType::Rook, is_white: false },
                position::Position { rank: 7, file: 1, piece_type: piece_type::PieceType::Knight, is_white: false },
                position::Position { rank: 7, file: 2, piece_type: piece_type::PieceType::Bishop, is_white: false },
                position::Position { rank: 7, file: 3, piece_type: piece_type::PieceType::Queen, is_white: false },
                position::Position { rank: 7, file: 4, piece_type: piece_type::PieceType::King, is_white: false },
                position::Position { rank: 7, file: 5, piece_type: piece_type::PieceType::Bishop, is_white: false },
                position::Position { rank: 7, file: 6, piece_type: piece_type::PieceType::Knight, is_white: false },
                position::Position { rank: 7, file: 7, piece_type: piece_type::PieceType::Rook, is_white: true },

            ]
            .to_vec(),
        }
    }

    pub fn positions(&self) -> JsValue {
        JsValue::from_serde(&self.positions).unwrap()
    }
    pub fn is_white_move(&self) -> bool {
        self.is_white_move
    }
    pub fn state(&self) -> JsValue {
        JsValue::from_serde(&self.state).unwrap()
    }
}