use crate::board::Board;
use crate::moves::{CastlingSide, Color, Move, MoveType, Piece, Square};
use crate::bitboards::{*}; 
//use crate::movegen::{get_legal_moves}; 

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ActionID(u32); 

pub const ACTION_SPACE_VERSION: u32 = 1;

const SQUARES: u32 = 64;
const FROM_BUCKET: u32 = SQUARES; 
const PROMO_BUCKET: u32 = SQUARES * SQUARES;
const PROMO_TYPES: u32 = 5;
const GET_PIECE_BITBOARD: [fn(&Board, Color) -> Bitboard; 6] = [
    Board::get_pawn_bb,
    Board::get_knight_bb,
    Board::get_bishop_bb,
    Board::get_rook_bb,
    Board::get_queen_bb,
    Board::get_king_bb,
]; 

/// size of policy vector
#[inline]
pub const fn num_actions() -> u32 {
    PROMO_BUCKET * PROMO_TYPES
}

/// maps promotion to code
#[inline]
fn promo_to_code(piece: Option<Piece>) -> u32 {
    match piece {
        None => 0,
        Some(Piece::Knight) => 1,
        Some(Piece::Bishop) => 2,
        Some(Piece::Rook) => 3,
        Some(Piece::Queen) => 4,
        _ => 0,
    }
}

/// maps promo-code to promotion piece
#[inline]
fn code_to_promo(code: u32) -> Option<Piece> {
    match code {
        0 => None,
        1 => Some(Piece::Knight),
        2 => Some(Piece::Bishop),
        3 => Some(Piece::Rook),
        4 => Some(Piece::Queen),
        _ => None,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MoveIntent {
    pub from: Square, 
    pub to: Square,
    pub promotion: Option<Piece>,
}

impl MoveIntent {
    pub fn from(&self) -> Square {
        self.from
    }

    pub fn to(&self) -> Square {
        self.to
    }

    pub fn promotion(&self) -> Option<Piece> {
        self.promotion
    }
}

/// encode an action
#[inline]
pub fn encode_action(mv: &Move) -> ActionID {
    let from = mv.from as u32;
    let to = mv.to as u32;

    let promo = match mv.move_type {
        MoveType::Promotion{ piece } => Some(piece),
        _ => None
    }; 

    let promo_code = promo_to_code(promo);
    let id = promo_code * PROMO_BUCKET + from * FROM_BUCKET + to; 
    ActionID(id)
}

/// decode an action into a MoveIntent
#[inline]
pub fn decode_action(action: ActionID) -> Result<MoveIntent, &'static str> {
    let id = action.0; 
    if id > num_actions() {
        return Err("Action ID invalid."); 
    }
    let promo = id / PROMO_BUCKET;
    let promo_piece = code_to_promo(promo); 
    let rem = id % PROMO_BUCKET;
    let from = rem / FROM_BUCKET; 
    let from_square = Square::try_from(from as u64).unwrap();
    let to = rem % FROM_BUCKET; 
    let to_square = Square::try_from(to as u64).unwrap(); 
    Ok(MoveIntent {
        from: from_square,
        to: to_square,
        promotion: promo_piece, 
    })

}

pub fn realize_move(board: &Board, intent: MoveIntent) -> Result<Move, &'static str> {
    // Normal, Castle { kingside: bool }, En Passant, DoublePawnPush, Promotion { piece: Piece }
    let piece = board.get_piece_at(intent.from() as u64).expect("move should be valid."); 

    let color = if GET_PIECE_BITBOARD[piece as usize](board, Color::White) & intent.from().to_bitboard() != 0 {
        Color::White
    } else {
        Color::Black
    }; 

    let from_row = (intent.from() as u8) / 8;
    let from_file = (intent.from() as u8) % 8;

    let to_row = (intent.to() as u8) / 8;
    let to_file = (intent.to() as u8) % 8;

    let drow = (to_row as i8)  - (from_row as i8);
    let dfile = (to_file as i8) - (from_file as i8); 

    if piece == Piece::King && intent.from() == KING_INITIAL_SQUARE[color as usize] && dfile.abs() == 2 {
        if intent.to() == CASTLING_DESTINATION_SQUARES[color as usize][CastlingSide::Queenside as usize] {
            return Ok(Move::new_castle(color, false))
        } else {
            return Ok(Move::new_castle(color, true))
        }
    }

    if piece == Piece::Pawn {
        if drow.abs() == 2 {
            return Ok(Move::new_double_pawn_push(intent.from(), intent.to(), color)); 
        }
        if intent.from().to_bitboard() & PAWN_PROMOTION_RANK[color as usize] != 0 {
            return Ok(Move::new_promotion(intent.from(), intent.to(), color, board.get_piece_at(intent.to() as u64), 
            intent.promotion().expect("move should be valid"))); 
        }
        if drow.abs() == 1 && dfile.abs() == 1 && board.get_piece_at(intent.to() as u64).is_none() {
            return Ok(Move::new_en_passant(intent.from(), intent.to(), color)); 
        }
    }

    Ok(Move::new_normal(
        intent.from(),
        intent.to(), 
        board.get_piece_at(intent.from() as u64).expect("move should be valid"),
        color, 
        board.get_piece_at(intent.to() as u64))
    )

}