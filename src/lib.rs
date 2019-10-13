#[macro_use]
extern crate serde_derive;

pub mod game;

use wasm_bindgen::prelude::*;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Testing out how to write vectors out
    let stuff = vec![5, 5, 2, 0, 8];

    for num in stuff {
        // Here each dom element can represent a place on the board and its
        // position should match up with the board to render it
        let val = document.create_element("div")?;
        val.set_inner_html(&num.to_string());
        body.append_child(&val)?;
    }

    // In theory with the positison can be displayed here with the dom
    // and and also event handlers can be added also. Question
    // why not just do that on the client?

    // I dont know right now what the right way is

    Ok(())
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
