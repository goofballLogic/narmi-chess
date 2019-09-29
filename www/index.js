import * as wasm from "narmi-chess";

console.log("Creating a new game");
const game = wasm.Game.new();
console.log(game);

console.log("The game is currently in state", game.state());
console.log("Possible games states are", wasm.GameState);

console.log("Is it white to move?", game.is_white_move());

console.log("Possible pieces:", wasm.PieceType);
console.log("Current positions:", game.positions());

const board = document.body.appendChild(document.createElement("DIV"));
board.classList.add("board");
let isWhite = true;
for(let rank = 0; rank < 8; rank++) {
    for(let file = 0; file < 8; file++) {
        const square = board.appendChild(document.createElement("DIV"));
        square.classList.add(isWhite ? "white" : "black");
        isWhite = !isWhite;
    }
    isWhite = !isWhite;
}

function findSquare(rank, file) {
    const index = ((7 - rank)*8)+file+1;
    return board.querySelector(`div:nth-child(${index})`);
}

console.log("Rank 0, File 1", findSquare(0, 1));
console.log("Rank 3, File 3", findSquare(3, 3));
console.log("Rank 7, File 7", findSquare(7, 7));

const pieceGlyphs = "♔♕♖♗♘♙♚♛♜♝♞♟";
console.log("Chess piece glyphs", pieceGlyphs)
game.positions().forEach(({ rank, file, piece_type, is_white}) => {
    const glyphIndex = (wasm.PieceType[piece_type] - 1) + (is_white ? 0 : 6);
    findSquare(rank, file).innerText = pieceGlyphs[glyphIndex];
});

