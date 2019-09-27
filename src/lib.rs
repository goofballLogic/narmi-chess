mod utils;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum PieceType {
    Empty = 0,
    King = 1,
    Queen = 2,
    Rook = 3,
    Bishop = 4,
    Knight = 5,
    Pawn = 6
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum GameState {
    NotStarted = 0,
    Started = 1,
    Stalemate = 2,
    WhiteResigned = 3,
    BlackResigned = 4,
    WhiteCheckmate = 5,
    BlackCheckmate = 6,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Position {
    rank: u8,
    file: u8,
    piece_type: PieceType,
    is_white: bool
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    positions: Vec<Position>,
    is_white_move: bool,
    state: GameState,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            is_white_move: true,
            state: GameState::NotStarted,
            positions: [
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
                Position { rank: 7, file: 7, piece_type: PieceType::Rook, is_white: false },

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
}

// #[derive(Serialize)]
// pub struct Example {
//     pub field1: HashMap<u32, String>,
//     pub field2: Vec<Vec<f32>>,
//     pub field3: [f32; 4],
// }

// #[wasm_bindgen]
// pub fn send_example_to_js() -> JsValue {
//     let mut field1 = HashMap::new();
//     field1.insert(0, String::from("ex"));
//     let example = Example {
//         field1,
//         field2: vec![vec![1., 2.], vec![3., 4.]],
//         field3: [1., 2., 3., 4.],
//     };

//     JsValue::from_serde(&example).unwrap()
// }
