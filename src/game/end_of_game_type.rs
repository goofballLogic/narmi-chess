#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum EndOfGameType {
    WhiteWin = 0,
    BlackWin = 1,
    Draw = 2,
}