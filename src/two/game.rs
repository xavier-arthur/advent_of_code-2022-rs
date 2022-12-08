#[derive(Debug, PartialEq)]
pub enum GameInput {
    A,
    B,
    C,
    X,
    Y,
    Z
}

#[derive(Debug)]
pub enum GameResult {
    WIN,
    LOSE,
    DRAW
}