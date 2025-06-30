use crate::bitboards::{*}; 
use crate::moves::{Color, Color::*, Piece::*, Move, Square, Square::*};

struct Board {
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


    /// Returns vector of white pawn moves - doesn't consider checks    
    fn get_white_pawn_moves(&self, color: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new(); 
        let pawn_bb: Bitboard = self.pieces[color as usize][Pawn as usize];
        let all_squares: Bitboard = self.get_all_pieces(); 
        let empty: Bitboard = !all_squares; 

        // Pushing pawns one square forward, deal with promotion later
        let one_step = pawn_bb.shift_north() & empty & !RANK_7; 

        // Pushing pawns two squares forward
        let two_step = one_step.shift_north() & empty & RANK_3;

        // Extracting moves from one_step
        let pawn_push_moves = Self::extract_pawn_push_moves(one_step); 
        moves.extend(pawn_push_moves); 


        moves
    }

    fn extract_pawn_push_moves(mut bb: Bitboard) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let from_bb = bb.shift_south(); 

        let mut to_bb = bb;

        while to_bb != 0 {
            let to_index = to_bb.trailing_zeros() as u8;
            to_bb = to_bb.clear_bit(to_index);
            let from_index = to_index - 8; 

            // Converting to piece enum
            let to = Square::try_from(to_index).unwrap();
            let from =  Square::try_from(from_index).unwrap();

            moves.push(Move::new_normal(
                from,
                to,
                Pawn,
                White,
                None,
            ))
        }
        moves
    }
}