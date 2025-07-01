use crate::bitboards::{*}; 
use crate::moves::{Color, Color::*, Piece, Piece::*, Square::*};

pub struct Board {
    pub pieces: [[Bitboard; 6]; 2],
    pub piece_lookup: [Piece; 64],
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

        let piece_lookup = [
            Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook,  
            Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn,          
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,   
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,  
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,   
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,   
            Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn,         
            Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook,  
        ];

        Board { pieces, piece_lookup }
    }

    /// Gets pieces for color 
    pub fn get_pieces(&self, color: Color) -> u64 {
        self.pieces[color as usize].iter().copied().reduce(|a, b| a | b).unwrap_or(0)
    }

    /// Returns position of all pieces
    pub fn get_all_pieces(&self) -> u64 {
        self.get_pieces(Black) | self.get_pieces(White)
    }

    // Try to make this branchless at some point 
    pub fn get_piece_at(&self, index: u64) -> Option<Piece> {
        match self.piece_lookup[index as usize] {
            Piece::Empty => None,
            piece => Some(piece),
        }
    }
}