use crate::bitboards::{*, self}; 

struct Board {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queen: u64,
    white_king: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queen: u64,
    black_king: u64,
}

impl Board {
    /// Creates a new board in the standard starting position.
    pub fn new() -> Board {
        Board {
            // White Pieces 
            white_pawns: RANK_2,
            white_knights: B1 | G1,
            white_bishops: C1 | F1,
            white_rooks: A1 | H1,
            white_king: E1,
            white_queen: D1,

            // Black Pieces
            black_pawns: RANK_7,
            black_knights: B8 | G8,
            black_bishops: C8 | F8,
            black_rooks: A8 | H8,
            black_queen: D8,
            black_king: E8,
        }
    }

    /// Returns position of all white pieces
    pub fn get_white_pieces(&self) -> u64 {
        self.white_pawns | self.white_knights | self.white_bishops |
        self.white_rooks | self.white_queen | self.white_king
    }

    /// Returns position of all black pieces
    pub fn get_black_pieces(&self) -> u64 {
        self.black_pawns | self.black_knights | self.black_bishops |
        self.black_rooks | self.black_queen | self.black_king
    }

    /// Returns position of all pieces
    pub fn get_all_pieces(&self) -> u64 {
        self.get_black_pieces() | self.get_white_pieces()
    }
}