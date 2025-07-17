use crate::{bitboards::FenError, board::Board}; 

enum Outcome {
    Draw = 0,
    Checkmate = 1,
}

struct GameState {
    board: Board,
    outcome: Option<Outcome>,
    past_states: Vec<Board>, // change this to some hashed version
}

impl GameState {
    pub fn new() -> Self {
        let board = Board::new();
        GameState {
            board,
            outcome: None,
            past_states: Vec::new(),
        }
    }

    pub fn from_fen(fen_string: &str) -> Result<Self, FenError> {
        let board = Board::from_fen(fen_string)?;
        Ok(GameState {
            board,
            outcome: None,
            past_states: Vec::new(),
        })
    }
}