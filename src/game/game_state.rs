use wasm_bindgen::prelude::*;

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