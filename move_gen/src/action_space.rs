use crate::board::Board;
use crate::moves::{Move, MoveType, Piece, Square};
//use crate::movegen::{get_legal_moves}; 

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ActionID(u32); 

pub const ACTION_SPACE_VERSION: u32 = 1;

const SQUARES: u32 = 64;
const FROM_BUCKET: u32 = SQUARES; 
const PROMO_BUCKET: u32 = SQUARES * SQUARES;
const PROMO_TYPES: u32 = 5;

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
    todo!()
}