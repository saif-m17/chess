use crate::board::Board;
use crate::moves::{Move, MoveType, Piece};
use crate::movegen::{get_legal_moves}; 

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ActionID(u32); 

pub const ACTION_SPACE_VERSION: u32 = 1;

const SQUARES: u32 = 64;
const FROM_BUCKET: u32 = SQUARES; 
const PROMO_BUCKET: u32 = SQUARES * SQUARES;
const PROMO_TYPES: u32 = 5;

/// size of policy vector
#[inline]
pub const fn num_actions() -> usize {
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
