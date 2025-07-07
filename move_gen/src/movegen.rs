use crate::attacktables::{ATTACK_TABLES, RAYS, RAYS_WITH_EDGE, ROOK_MAGICS, BISHOP_MAGICS};
use crate::bitboards::{*};
use crate::moves::{Color, Piece, Piece::*, Move, Square, Direction};
use crate::board::Board;

/// Returns vector of rook moves
pub fn get_rook_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut rook_bb = board.pieces[color as usize][Rook as usize];

    let enemies = board.get_pieces(color.opposite_color());
    let allies = board.get_pieces(color);
    let enemies_not_allies = enemies & !allies;
    let all_pieces = board.get_all_pieces(); 

    while rook_bb != 0 {
        let rook_index = rook_bb.trailing_zeros() as u64;
        let from = Square::try_from(rook_index).unwrap();
        rook_bb = rook_bb.clear_bit(rook_index); 

        let rook_magic = &ROOK_MAGICS[rook_index as usize]; 
        
        let blockers = rook_magic.direction_mask & all_pieces;
        let index = blockers.wrapping_mul(rook_magic.magic_num) >> (64 - rook_magic.index_bits);
        let mut attacks = rook_magic.attack_table[index as usize].unwrap();
        attacks = attacks & enemies_not_allies; 
        
        while attacks != 0 {
            let to_index = attacks.trailing_zeros() as u64;
            let to = Square::try_from(to_index).unwrap();
            attacks = attacks.clear_bit(to_index); 

            let captured_piece = board.get_piece_at(to_index); 
            moves.push(Move::new_normal(
                from,
                to,
                Rook,
                color,
                captured_piece,
            ))
        }
    }

    moves
}

pub fn get_bishop_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let mut bishop_bb = board.pieces[color as usize][Bishop as usize];

    let enemies = board.get_pieces(color.opposite_color());
    let allies = board.get_pieces(color);
    let enemies_not_allies = enemies & !allies;
    let all_pieces = board.get_all_pieces(); 

    while bishop_bb != 0 {
        let bishop_index = bishop_bb.trailing_zeros() as u64;
        let from = Square::try_from(bishop_index).unwrap();
        bishop_bb = bishop_bb.clear_bit(bishop_index); 

        let bishop_magic = &BISHOP_MAGICS[bishop_index as usize]; 
        
        let blockers = bishop_magic.direction_mask & all_pieces;
        let index = blockers.wrapping_mul(bishop_magic.magic_num) >> (64 - bishop_magic.index_bits);
        let mut attacks = bishop_magic.attack_table[index as usize].unwrap();
        attacks = attacks & enemies_not_allies; 
        
        while attacks != 0 {
            let to_index = attacks.trailing_zeros() as u64;
            let to = Square::try_from(to_index).unwrap();
            attacks = attacks.clear_bit(to_index); 
            
            let captured_piece = board.get_piece_at(to_index); 
            moves.push(Move::new_normal(
                from,
                to,
                Bishop,
                color,
                captured_piece,
            ))
        }
    }
    moves
}

/// Returns vector of king moves
pub fn get_king_moves(board: &Board, color: Color) -> Vec<Move>{
    let mut moves: Vec<Move> = Vec::new();
    let king_bb: Bitboard = board.pieces[color as usize][King as usize];
    let enemies = board.get_pieces(color.opposite_color()); 
    let allies = board.get_pieces(color); 
    let enemies_not_allies = enemies & !allies;

    let king_index = king_bb.trailing_zeros() as u64; 

    let from = Square::try_from(king_index).unwrap();

    let mut attacks = ATTACK_TABLES.king_attacks[king_index as usize] & enemies_not_allies; 

    while attacks != 0 {
        let to_index = attacks.trailing_zeros() as u64;
        attacks = attacks.clear_bit(to_index);
        let to = Square::try_from(to_index).unwrap();
        let captured_piece = board.get_piece_at(to_index); 
        moves.push(Move::new_normal(
            from,
            to,
            King,
            color,
            captured_piece,
        ))
    }
    moves
}

/// Returns vector of knight moves
pub fn get_knight_moves(board: &Board, color: Color) -> Vec<Move> {

    let mut moves: Vec<Move> = Vec::new();
    let mut knight_bb: Bitboard = board.pieces[color as usize][Knight as usize];
    let enemies = board.get_pieces(color.opposite_color()); 
    let allies = board.get_pieces(color); 
    let enemies_not_allies = enemies & !allies;

    while knight_bb != 0 {
        let knight = knight_bb.trailing_zeros() as u64;
        knight_bb = knight_bb.clear_bit(knight); 
        let from = Square::try_from(knight).unwrap();
        let mut attacks = ATTACK_TABLES.knight_attacks[knight as usize] & enemies_not_allies; 

        while attacks != 0 {
            let to_index = attacks.trailing_zeros() as u64;
            attacks = attacks.clear_bit(to_index);
            let to = Square::try_from(to_index).unwrap();
            let captured_piece = board.get_piece_at(to_index); 
            moves.push(Move::new_normal(
                from,
                to,
                Knight,
                color,
                captured_piece,
            ))
        }
    }
    moves
}

/// Returns vector of pawn moves - doesn't consider checks    
pub fn get_pawn_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new(); 
    let pawn_bb: Bitboard = board.pieces[color as usize][Pawn as usize];
    let all_squares: Bitboard = board.get_all_pieces(); 
    let empty: Bitboard = !all_squares; 

    // Pushing pawns one square forward, deal with promotion later
    let one_step = FORWARD_SHIFT[color as usize](pawn_bb) & empty; 

    // Pushing pawns two squares forward
    let two_step = FORWARD_SHIFT[color as usize](one_step) & empty & PAWN_DOUBLE_RANK[color as usize];

    // Extracting moves from one_step
    let pawn_push_moves = extract_pawn_push_moves(one_step, OFFSET_SINGLE_PUSH[color as usize], color);
    moves.extend(pawn_push_moves); 

    // Extracting moves from two_step
    let double_push_moves = extract_pawn_push_moves(two_step, OFFSET_DOUBLE_PUSH[color as usize], color);
    moves.extend(double_push_moves);

    // Pawn promotions not captures
    let promotions = PAWN_PROMOTION[color as usize](pawn_bb) & empty;
    let promo_pieces = [Queen, Rook, Bishop, Knight];
    for promote_piece in promo_pieces {
        moves.extend(extract_pawn_promotions(promotions, Some(promote_piece), color)); 
    }

    // Pawn attacks excluding ones that result in promotion
    let pawn_attacks_bbs = ATTACK_TABLES.pawn_attacks[color as usize];
    let enemies = board.get_pieces(color.opposite_color());
    let allies = board.get_pieces(color); 
    let enemies_not_allies = enemies & !allies;
    let exclude_promotions = pawn_bb & !PAWN_PROMOTION_RANK[color as usize]; 
    let pawn_attack_moves = extract_pawn_attack_moves(board, exclude_promotions, &pawn_attacks_bbs, enemies_not_allies, color); 
    moves.extend(pawn_attack_moves);

    // Pawn promotions that result in captures
    let promotion_elligible = pawn_bb & PAWN_DOUBLE_RANK[color as usize];
    for promote_piece in promo_pieces {
        moves.extend(extract_pawn_promotion_captures(board, promotion_elligible, &pawn_attacks_bbs, 
            Some(promote_piece), enemies_not_allies, color)); 
    } 

    moves
}

fn extract_pawn_push_moves(bb: Bitboard, offset: i8, color:Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let mut to_bb = bb;

    while to_bb != 0 {
        let to_index = to_bb.trailing_zeros() as u64;
        to_bb = to_bb.clear_bit(to_index);
        let from_index = (to_index as i64 - offset as i64) as u64; 

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

/// Extracts the pawn attack moves from without considering attacks that lead to promotion (from attack tables)
/// no en passent yet
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

/// Extracting promotion moves - separate function to avoid branching for efficiency reasons
fn extract_pawn_promotions(bb: Bitboard, promote_piece: Option<Piece>, color: Color) -> Vec<Move> {
    let mut to_bb = bb;
    let mut moves: Vec<Move> = Vec::new();

    while to_bb != 0 {
        let to_index = to_bb.trailing_zeros() as u64;
        to_bb = to_bb.clear_bit(to_index);
        let from_index = to_index - 8;

        let to = Square::try_from(to_index).unwrap();
        let from = Square::try_from(from_index).unwrap();

        moves.push(Move::new_promotion(
            from,
            to,
            color,
            None,
            promote_piece.expect("Passing piece explicitly"),
        )); 
    }
    moves
}

/// Extracts moves where we promote and capture. 
fn extract_pawn_promotion_captures(board: &Board, bb: Bitboard, attacks: &[Bitboard; 64], promote_piece: Option<Piece>, 
    enemies_not_allies: Bitboard, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new(); 
    let mut from_bb = bb;

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
            moves.push(Move::new_promotion(
                from,
                to,
                color,
                captured_piece,
                promote_piece.expect("Piece passed explicitly."),
            ))
        } 
    }
    moves
}

/// Returns bitboard of rays in all directions given in directions_list originating from square.
/// Doesnt include edge squares - used for magic index. 
fn get_directions_bb(square: Square, directions_list: Vec<Direction>) -> Bitboard {
    let mut directions_bb = 0u64;
    for direction in directions_list {
        directions_bb |= RAYS[square as usize][direction as usize]; 
    }
    directions_bb
    
}