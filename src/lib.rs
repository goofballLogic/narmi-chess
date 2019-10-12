#[macro_use]
extern crate serde_derive;

<<<<<<< HEAD
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
=======
pub mod game;

//use wasm_bindgen::prelude::*;
>>>>>>> 107fc245d21674e341dda618c2cfa56cce4c0ed6

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

<<<<<<< HEAD
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    Empty = 0,
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
    IsWhite = 8,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct Position {
    rank: u8,
    file: u8,
    piece: Piece,
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
            positions: [Position {
                rank: 0,
                file: 0,
                piece: Piece::Rook,
            }]
            .to_vec(),
        }
    }

    pub fn positions(&self) -> *const Position {
        self.positions.as_ptr()
    }
    pub fn is_white_move(&self) -> bool {
        self.is_white_move
    }
    pub fn state(&self) -> GameState {
        self.state
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
    #[wasm_bindgen(js_namespace = Date, js_name = now)]
    fn date_now() -> f64;
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    console_log(&format_args!("Hello {} at {}", name, date_now()).to_string());
}

#[derive(Serialize)]
pub struct Example {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>,
    pub field3: [f32; 4],
}

#[wasm_bindgen]
pub fn send_example_to_js() -> JsValue {
    let mut field1 = HashMap::new();
    field1.insert(0, String::from("ex"));
    let example = Example {
        field1,
        field2: vec![vec![1., 2.], vec![3., 4.]],
        field3: [1., 2., 3., 4.],
    };

    JsValue::from_serde(&example).unwrap()
}
=======
>>>>>>> 107fc245d21674e341dda618c2cfa56cce4c0ed6
