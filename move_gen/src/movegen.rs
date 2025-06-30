use crate::bitboards::{*};
use crate::moves::{Color, Color::*, Piece::*, Move, Square, Square::*};
use crate::board::Board; 

/// Returns vector of white pawn moves - doesn't consider checks    
pub fn get_white_pawn_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new(); 
    let pawn_bb: Bitboard = board.pieces[color as usize][Pawn as usize];
    let all_squares: Bitboard = board.get_all_pieces(); 
    let empty: Bitboard = !all_squares; 

    // Pushing pawns one square forward, deal with promotion later
    let one_step = pawn_bb.shift_north() & empty & !RANK_7; 

    // Pushing pawns two squares forward
    let two_step = one_step.shift_north() & empty & RANK_3;

    // Extracting moves from one_step
    let pawn_push_moves = extract_pawn_push_moves(one_step); 
    moves.extend(pawn_push_moves); 


    moves
}

pub fn extract_pawn_push_moves(mut bb: Bitboard) -> Vec<Move> {
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