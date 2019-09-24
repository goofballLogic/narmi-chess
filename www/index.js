import * as wasm from "narmi-chess";
import { memory } from "narmi-chess/narmi_chess_bg";

wasm.greet("Imran");
console.log("WASM", wasm);
const game = wasm.Game.new();
console.log(game);
console.log(game.is_white_move());
const positionsPointer = game.positions();
const positions = new Uint8Array(memory.buffer, positionsPointer, 3 * 16); // not sure how many positions we have???
console.log(positions);
let example = wasm.send_example_to_js();
console.log(example);
