use crate::attacktables::{ATTACK_TABLES, RAYS, ROOK_MAGICS, BISHOP_MAGICS, Magic, IN_BETWEEN_SQUARES};
use crate::bitboards::{*};
use crate::moves::{Color, Piece, Piece::*, Move, Square, Direction};
use crate::board::Board;
use crate::utils::MoveList;

/// Returns a boolean check if the move puts the king in check.
pub fn moves_into_check(board: &Board, color: Color, mve: &Move) -> bool {
    let new_board_pieces = board.make_shallow_move(mve);
    let king_index = new_board_pieces[color as usize][King as usize].trailing_zeros() as u64; 
    let king_square = Square::try_from(king_index).unwrap();
    is_attacked(new_board_pieces, king_square, color)
    
}

/// Returns fully legal moves
pub fn get_legal_moves(board: &Board, color: Color, moves: &mut MoveList) {
    get_pseudo_legal_moves(board, color, moves);
    moves.retain(|mve| !moves_into_check(board, color, &mve));
}

/// Returns a boolean indicating whether the move is legal 
pub fn is_move_legal(board: &Board, color: Color, mve: &Move) -> bool {
    let into_check = moves_into_check(board, color, mve);
    let mut possible_moves = MoveList::new();  
    get_check_aware_pseudo_legal_moves(board, color, &mut possible_moves);
    possible_moves.contains(mve) && !into_check 
}

/// Returns vector of legal moves
pub fn get_check_aware_pseudo_legal_moves(board: &Board, color: Color, moves: &mut MoveList) {
    let king_bb = board.pieces[color as usize][King as usize]; 
    let king_index = king_bb.trailing_zeros() as u64; 
    let king_square = Square::try_from(king_index).unwrap();

    let (checkers, num_checkers) = get_attackers(board, color, king_square);

    if num_checkers > 1 {
        get_pseudo_king_moves(board, color, moves);

    } else if num_checkers == 1 {
        let checker_index = checkers.trailing_zeros();
        let between_checker_and_king = IN_BETWEEN_SQUARES[king_square as usize][checker_index as usize];
        let valid_destinations = between_checker_and_king | checkers;
        get_pseudo_king_moves(board, color, moves); 
        get_pseudo_queen_moves(board, color, valid_destinations, moves);
        get_pseudo_rook_moves(board, color, valid_destinations, moves);
        get_pseudo_knight_moves(board, color, valid_destinations, moves);
        get_pseudo_bishop_moves(board, color, valid_destinations, moves);
        get_pseudo_pawn_moves(board, color, valid_destinations, moves);

    } else {
        get_pseudo_legal_moves(board, color, moves); 
    }
}

/// Returns vector of all pseudo-legal moves
pub fn get_pseudo_legal_moves(board: &Board, color: Color, moves: &mut MoveList)  {

    get_pseudo_pawn_moves(board, color, !0u64, moves);
    get_pseudo_knight_moves(board, color, !0u64, moves);
    get_pseudo_bishop_moves(board, color, !0u64, moves);
    get_pseudo_rook_moves(board, color, !0u64, moves);
    get_pseudo_queen_moves(board, color, !0u64, moves);
    get_pseudo_king_moves(board, color, moves);
    get_castling_moves(board, color, moves); 

}

/// Returns vector of pseudo-legal queen moves
pub fn get_pseudo_queen_moves(board: &Board, color: Color, valid_destinations: Bitboard, moves: &mut MoveList) {
    let queen_bb = board.pieces[color as usize][Queen as usize];
    let queen_index = queen_bb.trailing_zeros() as u64;

    if queen_index == 64 {
        return; 
    } 

    let from = Square::try_from(queen_index).unwrap();

    let enemies = board.get_pieces(color.opposite_color()); 
    let allies = board.get_pieces(color);

    // Getting moves in rook directions
    let mut rook_attacks = get_sliding_piece_attacks(enemies | allies, from, &ROOK_MAGICS); 
    rook_attacks = rook_attacks & !allies & valid_destinations;

    extract_moves(board, color, rook_attacks, from, Queen, moves); 

    // Getting moves in bishop directions
    let mut bishop_attacks = get_sliding_piece_attacks(enemies | allies, from, &BISHOP_MAGICS); 
    bishop_attacks = bishop_attacks & !allies & valid_destinations;

    extract_moves(board, color, bishop_attacks, from, Queen, moves);

}

/// Returns vector of pseudo-legal rook moves
pub fn get_pseudo_rook_moves(board: &Board, color: Color, valid_destinations: Bitboard, moves: &mut MoveList) {
    let mut rook_bb = board.pieces[color as usize][Rook as usize];

    let allies = board.get_pieces(color);
    let enemies = board.get_pieces(color.opposite_color()); 

    while rook_bb != 0 {
        let rook_index = rook_bb.trailing_zeros() as u64;
        let from = Square::try_from(rook_index).unwrap();
        rook_bb.clear_bit(rook_index); 

        let mut attacks = get_sliding_piece_attacks(enemies | allies, from, &ROOK_MAGICS); 
        attacks = attacks & !allies & valid_destinations;

        extract_moves(board, color, attacks, from, Rook, moves); 
        
    }
}

/// Returns vector of pseud-legal bishop moves.
pub fn get_pseudo_bishop_moves(board: &Board, color: Color, valid_destinations: Bitboard, moves: &mut MoveList) {
    let mut bishop_bb = board.pieces[color as usize][Bishop as usize];

    let allies = board.get_pieces(color);
    let enemies = board.get_pieces(color.opposite_color()); 

    while bishop_bb != 0 {
        let bishop_index = bishop_bb.trailing_zeros() as u64;
        let from = Square::try_from(bishop_index).unwrap();
        bishop_bb.clear_bit(bishop_index); 

        let mut attacks = get_sliding_piece_attacks(enemies | allies, from, &BISHOP_MAGICS); 
        attacks = attacks & !allies & valid_destinations; 

        extract_moves(board, color, attacks, from, Bishop, moves); 
        
    }
}

/// Returns vector of pseudo-legal king moves
pub fn get_pseudo_king_moves(board: &Board, color: Color, moves: &mut MoveList) {
    let king_bb: Bitboard = board.pieces[color as usize][King as usize];
    let allies = board.get_pieces(color); 

    let king_index = king_bb.trailing_zeros() as u64; 

    let from = Square::try_from(king_index).unwrap();

    let attacks = ATTACK_TABLES.king_attacks[king_index as usize] & !allies; 

    extract_moves(board, color, attacks, from, King, moves);

}

/// Returns castling moves for color (assumes we are not in check).
pub fn get_castling_moves(board: &Board, color: Color, moves: &mut MoveList) {
    let king_init = KING_INITIAL_SQUARE[color as usize];
    let queenside_rook_init = ROOK_CASTLING_INITIAL_SQUARE[color as usize][0];
    let kingside_rook_init = ROOK_CASTLING_INITIAL_SQUARE[color as usize][1];

    let king_bb = board.pieces[color as usize][King as usize];
    let rook_bb = board.pieces[color as usize][Rook as usize];

    let checked = is_in_check(board, color); 

    if !king_bb.get_bit(king_init as u64) || checked {
        return; 
    }

    if rook_bb.get_bit(queenside_rook_init as u64) && board.can_castle_queenside(color) {
        moves.push(Move::new_castle(color, false)); 
    }

    if rook_bb.get_bit(kingside_rook_init as u64) && board.can_castle_kingside(color) {
        moves.push(Move::new_castle(color, true)); 
    }

}

/// Returns vector of pseudo-legal knight moves
pub fn get_pseudo_knight_moves(board: &Board, color: Color, valid_destinations: Bitboard, moves: &mut MoveList) {

    let mut knight_bb: Bitboard = board.pieces[color as usize][Knight as usize];
    let allies = board.get_pieces(color); 

    while knight_bb != 0 {
        let knight = knight_bb.trailing_zeros() as u64;
        knight_bb.clear_bit(knight); 
        let from = Square::try_from(knight).unwrap();
        let attacks = ATTACK_TABLES.knight_attacks[knight as usize] & !allies & valid_destinations; 

        extract_moves(board, color, attacks, from, Knight, moves); 

    }
}

/// Returns vector of pseudo-legal pawn moves  
pub fn get_pseudo_pawn_moves(board: &Board, color: Color, valid_destinations: Bitboard, moves: &mut MoveList) {
    let pawn_bb: Bitboard = board.pieces[color as usize][Pawn as usize];
    let all_squares: Bitboard = board.get_all_pieces(); 
    let empty: Bitboard = !all_squares; 

    // Pushing pawns one square forward
    let one_step = FORWARD_SHIFT[color as usize](pawn_bb & !PAWN_PROMOTION_RANK[color as usize]) & empty & valid_destinations; 

    // Pushing pawns two squares forward
    let two_step = FORWARD_SHIFT[color as usize](one_step & PAWN_DOUBLE_RANK[color as usize]) & empty  & valid_destinations;

    // Extracting moves from one_step
    extract_pawn_push_moves(one_step, OFFSET_SINGLE_PUSH[color as usize], color, moves);

    // Extracting moves from two_step
    extract_pawn_double_push_moves(two_step, OFFSET_DOUBLE_PUSH[color as usize], color, moves);

    // Pawn promotions not captures
    let promotions = PAWN_PROMOTION[color as usize](pawn_bb) & empty & valid_destinations;
    let promo_pieces = [Queen, Rook, Bishop, Knight];
    for promote_piece in promo_pieces {
        extract_pawn_promotions(promotions, Some(promote_piece), color, moves); 
    }

    // Pawn attacks excluding ones that result in promotion
    let pawn_attacks_bbs = ATTACK_TABLES.pawn_attacks[color as usize];
    let enemies = board.get_pieces(color.opposite_color());
    let allies = board.get_pieces(color); 
    let enemies_not_allies = enemies & !allies;
    let exclude_promotions = pawn_bb & !PAWN_PROMOTION_RANK[color as usize]; 
    extract_pawn_attack_moves(board, exclude_promotions, &pawn_attacks_bbs, 
        enemies_not_allies, color, valid_destinations, moves); 

    // Pawn promotions that result in captures
    let promotion_elligible = pawn_bb & PAWN_PROMOTION_RANK[color as usize];
    for promote_piece in promo_pieces {
        extract_pawn_promotion_captures(board, promotion_elligible, &pawn_attacks_bbs, 
            Some(promote_piece), enemies_not_allies, color, valid_destinations, moves); 
    }   

    // En passant
    extract_en_passant_moves(board, color, pawn_bb, valid_destinations, moves); 

}

fn extract_pawn_push_moves(bb: Bitboard, offset: i8, color:Color, moves: &mut MoveList) {
    let mut to_bb = bb;

    while to_bb != 0 {
        let to_index = to_bb.trailing_zeros() as u64;
        to_bb.clear_bit(to_index);
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
}

fn extract_pawn_double_push_moves(bb: Bitboard, offset: i8, color:Color, moves: &mut MoveList) {
    let mut to_bb = bb;

    while to_bb != 0 {
        let to_index = to_bb.trailing_zeros() as u64;
        to_bb.clear_bit(to_index);
        let from_index = (to_index as i64 - offset as i64) as u64; 

        let to = Square::try_from(to_index).unwrap();
        let from =  Square::try_from(from_index).unwrap();

        moves.push(Move::new_double_pawn_push(
            from,
            to,
            color,
        ))
    }
}

// TODO FIX THIS TO CONSIDER SQUARES ATTACKED VIA THE OPPOSITE COLOR 
/// Extracts the pawn attack moves from without considering attacks that lead to promotion (from attack tables)
fn extract_pawn_attack_moves(board: &Board, pawnbb: Bitboard, attacks: &[Bitboard; 64], 
    enemies_not_allies: Bitboard, color: Color, valid_destinations: Bitboard, moves: &mut MoveList) {

    let attack_targets = enemies_not_allies & valid_destinations;

    if attack_targets == 0 {
        return; 
    }

    let mut from_bb = pawnbb;

    while from_bb != 0 {
        let from_index = from_bb.trailing_zeros() as u64;
        from_bb.clear_bit(from_index);

        let from = Square::try_from(from_index).unwrap(); 

        let mut to_bb = attacks[from_index as usize] & attack_targets;
        while to_bb != 0 {
            let to_index = to_bb.trailing_zeros() as u64;
            to_bb.clear_bit(to_index);
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
}

/// Extracting promotion moves - separate function to avoid branching for efficiency reasons
fn extract_pawn_promotions(bb: Bitboard, promote_piece: Option<Piece>, color: Color, moves: &mut MoveList) {
    let mut to_bb = bb;

    while to_bb != 0 {
        let to_index = to_bb.trailing_zeros() as u64;
        to_bb.clear_bit(to_index);
        let from_index = to_index as i8 - OFFSET_SINGLE_PUSH[color as usize];

        let to = Square::try_from(to_index).unwrap();
        let from = Square::try_from(from_index as u64).unwrap();

        moves.push(Move::new_promotion(
            from,
            to,
            color,
            None,
            promote_piece.expect("Passing piece explicitly"),
        )); 
    }
}

/// Extracts moves where we promote and capture. 
fn extract_pawn_promotion_captures(board: &Board, bb: Bitboard, attacks: &[Bitboard; 64], promote_piece: Option<Piece>, 
    enemies_not_allies: Bitboard, color: Color, valid_destinations: Bitboard, moves: &mut MoveList)  {
    let mut from_bb = bb;

    while from_bb != 0 {
        let from_index = from_bb.trailing_zeros() as u64;
        from_bb.clear_bit(from_index);

        let from = Square::try_from(from_index).unwrap(); 

        let mut to_bb = attacks[from_index as usize] & enemies_not_allies & valid_destinations;
        while to_bb != 0 {
            let to_index = to_bb.trailing_zeros() as u64;
            to_bb.clear_bit(to_index);
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
}

/// Extract en passant moves
fn extract_en_passant_moves(board: &Board, color: Color, pawns: Bitboard, valid_destinations: Bitboard, moves: &mut MoveList) {
    if let Some(ep_square) = board.en_passant_square { 
        let mut ep_attackers = pawns & ATTACK_TABLES.pawn_attacks[color.opposite_color() as usize][ep_square as usize]; 
        let is_ep_valid = valid_destinations.get_bit(ep_square as u64); 
        if is_ep_valid {
            while ep_attackers != 0 {
                let ep_from = ep_attackers.trailing_zeros();
                ep_attackers.clear_bit(ep_from as u64);
                let from = Square::try_from(ep_from as u64).unwrap();
                moves.push(Move::new_en_passant(
                    from,
                    ep_square,
                    color,
                ))
            }
        }
    } 

}

/// Returns bitboard of rays in all directions given in directions_list originating from square.
/// Doesnt include edge squares - used for magic index. 
fn _get_directions_bb(square: Square, directions_list: Vec<Direction>) -> Bitboard {
    let mut directions_bb = 0u64;
    for direction in directions_list {
        directions_bb |= RAYS[square as usize][direction as usize]; 
    }
    directions_bb
    
}

/// Finds pinned pieces on the given board for a given color, assuming checking pieces
/// located on the given bitboard.
fn _find_pinned_pieces(board: &Board, color: Color) -> Bitboard {
    let all_pieces = board.get_all_pieces(); 
    let ally_pieces = board.get_pieces(color); 
    let king_bb = board.pieces[color as usize][King as usize];
    let king_index = king_bb.trailing_zeros() as usize;  

    let mut pinned_pieces = 0u64;

    // Pieces pinned by the queen
    let enemy_queen = board.pieces[color.opposite_color() as usize][Queen as usize];
    let enemy_queen_index = enemy_queen.trailing_zeros() as usize;

    let squares_between_queen = IN_BETWEEN_SQUARES[king_index][enemy_queen_index]; 
    let pieces_between_queen = squares_between_queen & all_pieces;

    if pieces_between_queen.count_ones() == 1 {
        let ally_pieces_between_queen = squares_between_queen & ally_pieces; 
        pinned_pieces |= ally_pieces_between_queen; 
    }

    // Pieces pinned by bishops 
    let mut enemy_bishops = board.pieces[color.opposite_color() as usize][Bishop as usize];

    while enemy_bishops != 0 {
        let bishop_index = enemy_bishops.trailing_zeros() as usize;
        enemy_bishops.clear_bit(bishop_index as u64);

        let squares_between_bishop = IN_BETWEEN_SQUARES[king_index][bishop_index];
        let pieces_between_bishop = squares_between_bishop & all_pieces; 
        if pieces_between_bishop.count_ones() == 1 {
            let ally_pieces_between_bishop = squares_between_bishop & ally_pieces;
            pinned_pieces |= ally_pieces_between_bishop; 
        }
    }

    // Pieces pinned by rooks 
    let mut enemy_rooks = board.pieces[color.opposite_color() as usize][Rook as usize]; 


    while enemy_rooks != 0 {
        let rook_index = enemy_rooks.trailing_zeros() as usize;
        enemy_rooks.clear_bit(rook_index as u64);

        let squares_between_rook = IN_BETWEEN_SQUARES[king_index][rook_index];
        let pieces_between_rook = squares_between_rook & all_pieces; 
        if pieces_between_rook.count_ones() == 1 {
            let ally_pieces_between_rook = squares_between_rook & ally_pieces;
            pinned_pieces |= ally_pieces_between_rook; 
        }
    }

    pinned_pieces

}

/// Checks if square is attacked (attackers - bitboard of attackers, number of pieces that attack
/// said square). 
fn get_attackers(board: &Board, color: Color, square:Square) -> (Bitboard, u32) {
    let enemy_color = color.opposite_color(); 
    let mut attackers = 0u64; 
    let enemies = board.get_pieces(enemy_color); 
    let all_pieces = board.get_all_pieces();

    attackers |= ATTACK_TABLES.pawn_attacks[color as usize][square as usize] & enemies;
    attackers |= ATTACK_TABLES.knight_attacks[square as usize] & enemies;
    attackers |= ATTACK_TABLES.king_attacks[square as usize] & enemies;
    attackers |= get_sliding_piece_attacks(all_pieces, square, &ROOK_MAGICS) & enemies;
    attackers |= get_sliding_piece_attacks(all_pieces, square, &BISHOP_MAGICS) & enemies;

    (attackers, attackers.count_ones())
}

/// Check's whether color's piece on square is attacked. 
pub fn is_attacked(pieces: [[Bitboard; 6]; 2], square: Square, color: Color) -> bool {
    let mut attackers = 0u64; 
    let enemies_bbs = pieces[color.opposite_color() as usize];
    let all_pieces = get_all_pieces_from_bbs(pieces); 

    attackers |= ATTACK_TABLES.pawn_attacks[color as usize][square as usize] & enemies_bbs[Pawn as usize];
    attackers |= ATTACK_TABLES.knight_attacks[square as usize] & enemies_bbs[Knight as usize];
    attackers |= ATTACK_TABLES.king_attacks[square as usize] & enemies_bbs[King as usize];
    attackers |= get_sliding_piece_attacks(
        all_pieces, 
        square, 
        &ROOK_MAGICS) 
        & (enemies_bbs[Rook as usize] | enemies_bbs[Queen as usize]);
    attackers |= get_sliding_piece_attacks(
        all_pieces,
        square, 
        &BISHOP_MAGICS) 
        & (enemies_bbs[Bishop as usize] | enemies_bbs[Queen as usize]);

    attackers != 0
}

/// Get all pieces from a list of all piece bitboards rather than board object
fn get_all_pieces_from_bbs(pieces: [[Bitboard; 6]; 2]) -> Bitboard {
    pieces[0][0] | pieces[0][1] | pieces[0][2] | 
    pieces[0][3] | pieces[0][4] | pieces[0][5] |
    pieces[1][0] | pieces[1][1] | pieces[1][2] | 
    pieces[1][3] | pieces[1][4] | pieces[1][5] 

}

/// Returns bitboard of attacks from sliding piece at square, given the magic table to look at
pub fn get_sliding_piece_attacks(all_pieces: Bitboard, square: Square, magic_table: &[Magic; 64]) -> Bitboard {
    let magic = &magic_table[square as usize];
    let blockers = magic.direction_mask & all_pieces;
    let index = blockers.wrapping_mul(magic.magic_num) >> (64 - magic.index_bits);
    let attacks = magic.attack_table[index as usize].unwrap();
    attacks
}

/// Extract moves from an attacks bitboard for new normal move.
fn extract_moves(board: &Board, color: Color, attacks_fixed: Bitboard, from: Square, piece: Piece, moves: &mut MoveList) {
    let mut attacks = attacks_fixed; 

    while attacks != 0 {
        let to_index = attacks.trailing_zeros() as u64;
        attacks.clear_bit(to_index);
        let to = Square::try_from(to_index).unwrap();

        let captured_piece = board.get_piece_at(to_index);
        moves.push(Move::new_normal(
            from,
            to, 
            piece,
            color,
            captured_piece,
        )) 
    }
}

pub fn is_in_check(board: &Board, color: Color) -> bool {
    let king_index = board.pieces[color as usize][King as usize].trailing_zeros() as u64; 
    let king_square = Square::try_from(king_index).unwrap(); 
    is_attacked(board.pieces, king_square, color)
}