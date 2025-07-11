use crate::bitboards::{*}; 
use crate::moves::{Color, Color::*, Piece, Piece::*, Square::*, Move};

pub struct Board {
    pub pieces: [[Bitboard; 6]; 2],
    pub piece_lookup: [Option<Piece>; 64],
    pub past_moves: Vec<Move>,
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

        let piece_lookup: [Option<Piece>; 64] = [
            Some(Rook), Some(Knight), Some(Bishop), Some(Queen), Some(King), Some(Bishop), Some(Knight), Some(Rook),  
            Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn),          
            None, None, None, None, None, None, None, None,   
            None, None, None, None, None, None, None, None,  
            None, None, None, None, None, None, None, None,   
            None, None, None, None, None, None, None, None,   
            Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn),         
            Some(Rook), Some(Knight), Some(Bishop), Some(Queen), Some(King), Some(Bishop), Some(Knight), Some(Rook),  
        ];

        let past_moves: Vec<Move> = Vec::new(); 

        Board { pieces, piece_lookup, past_moves }
    }

    /// Gets pieces for color 
    pub fn get_pieces(&self, color: Color) -> u64 {
        self.pieces[color as usize].iter().copied().reduce(|a, b| a | b).unwrap_or(0)
    }

    /// Returns position of all pieces
    pub fn get_all_pieces(&self) -> u64 {
        self.get_pieces(Black) | self.get_pieces(White)
    }

    /// Returns piece at a given index if it exists, else None
    pub fn get_piece_at(&self, index: u64) -> Option<Piece> {
        self.piece_lookup[index as usize]
    }

    /// Returns bitboard of all sliding pieces for color.
    pub fn get_sliding_pieces(&self, color: Color) -> Bitboard {
        self.pieces[color as usize][Rook as usize] |
        self.pieces[color as usize][Bishop as usize] |
        self.pieces[color as usize][Queen as usize]
    }

}