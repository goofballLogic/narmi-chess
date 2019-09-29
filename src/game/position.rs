use wasm_bindgen::prelude::*;
use super::piece_type;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Position {
    pub rank: u8,
    pub file: u8,
    pub piece_type: piece_type::PieceType,
    pub is_white: bool
}
