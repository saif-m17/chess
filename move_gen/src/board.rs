use crate::bitboards::{*}; 
use crate::moves::{Color, Color::*, Piece::*, Move, Square, Square::*};

pub struct Board {
    pub pieces: [[Bitboard; 6]; 2],
}

impl Board {
    /// Creates a new board in the standard starting position.
    pub fn new() -> Board {
        let mut pieces = [[0u64;  6]; 2]; 
            
        // White Pieces 
        pieces[White as usize][Pawn as usize] = RANK_2;
        pieces[White as usize][Knight as usize] = B1.to_bitboard() | G1.to_bitboard();
        pieces[White as usize][Bishop as usize] = C1.to_bitboard() | F1.to_bitboard(); 
        pieces[White as usize][Rook as usize] = A1.to_bitboard() | H1.to_bitboard();
        pieces[White as usize][King as usize] = E1.to_bitboard();
        pieces[White as usize][Queen as usize] = D1.to_bitboard();

        // Black Pieces
        pieces[Black as usize][Pawn as usize] = RANK_2;
        pieces[Black as usize][Knight as usize] = B1.to_bitboard() | G1.to_bitboard();
        pieces[Black as usize][Bishop as usize] = C1.to_bitboard() | F1.to_bitboard(); 
        pieces[Black as usize][Rook as usize] = A1.to_bitboard() | H1.to_bitboard();
        pieces[Black as usize][King as usize] = E1.to_bitboard();
        pieces[Black as usize][Queen as usize] = D1.to_bitboard();

        Board { pieces }
    }

    /// Gets pieces for color 
    pub fn get_pieces(&self, color: Color) -> u64 {
        self.pieces[color as usize].iter().copied().reduce(|a, b| a | b).unwrap_or(0)
    }

    /// Returns position of all pieces
    pub fn get_all_pieces(&self) -> u64 {
        self.get_pieces(Black) | self.get_pieces(White)
    }
}