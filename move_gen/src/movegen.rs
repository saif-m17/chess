use crate::attacktables::AttackTables;
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
    // TODO: change so it deals with both colors without branching - just use functions from bitboards.rs
    let one_step = FORWARD_SHIFT[color as usize](pawn_bb) & empty; 

    // Pushing pawns two squares forward
    let two_step = FORWARD_SHIFT[color as usize](one_step) & empty & PAWN_DOUBLE_RANK[color as usize];

    // Extracting moves from one_step
    let pawn_push_moves = extract_pawn_push_moves(one_step, 8, color); 
    moves.extend(pawn_push_moves); 

    // Extracting moves from two_step
    let push_double_push_moves = extract_pawn_push_moves(two_step, 16, color);
    moves.extend(push_double_push_moves);

    // Pawn promotions (TODO)

    // Pawn attacks 
    let pawn_attacks_bbs = AttackTables::get().pawn_attacks[color as usize];
    let enemies = board.get_pieces(color.opposite_color());
    let allies = board.get_pieces(color); 
    let enemies_not_allies = enemies & !allies; 
    let pawn_attack_moves = extract_pawn_attack_moves(board, pawn_bb, &pawn_attacks_bbs, enemies_not_allies, color); 
    moves.extend(pawn_attack_moves); 

    moves
}

fn extract_pawn_push_moves(bb: Bitboard, offset: u64, color:Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let mut to_bb = bb;

    while to_bb != 0 {
        let to_index = to_bb.trailing_zeros() as u64;
        to_bb = to_bb.clear_bit(to_index);
        let from_index = to_index - offset; 

        // Converting to piece enum
        let to = Square::try_from(to_index).unwrap();
        let from =  Square::try_from(from_index).unwrap();

        moves.push(Move::new_normal(
            from,
            to,
            Pawn,
            color,
            None,
        ))
    }
    moves
}

/// Extracts the pawn attack moves from 
fn extract_pawn_attack_moves(board: &Board, pawnbb: Bitboard, attacks: &[Bitboard; 64], 
    enemies_not_allies: Bitboard, color: Color) -> Vec<Move> {

    let mut moves: Vec<Move> = Vec::new();
    let mut from_bb = pawnbb; 

    while from_bb != 0 {
        let from_index = from_bb.trailing_zeros() as u64;
        from_bb = from_bb.clear_bit(from_index);

        let from = Square::try_from(from_index).unwrap(); 

        let mut to_bb = attacks[from_index as usize] & enemies_not_allies;
        while to_bb != 0 {
            let to_index = to_bb.trailing_zeros() as u64;
            to_bb = to_bb.clear_bit(to_index);
            let to = Square::try_from(to_index).unwrap();

            let captured_piece = board.get_piece_at(to_index); 
            moves.push(Move::new_normal(
                from,
                to,
                Pawn,
                color,
                captured_piece,
            ))
        } 
    }

    moves
}