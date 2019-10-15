#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum PieceType {
    Empty = 0,
    King = 1,
    Queen = 2,
    Rook = 3,
    Bishop = 4,
    Knight = 5,
    Pawn = 6
}