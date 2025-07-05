use crate::bitboards::Bitboard;
use crate::moves::Direction; 

mod rays_table;
pub use rays_table::RAYS;

mod basic_attacktables;
pub use basic_attacktables::ATTACK_TABLES; 

pub const BISHOP_INDEX_BITS: usize = 9;
pub const ROOK_INDEX_BITS: usize = 12;

pub struct AttackTables {
    pub pawn_attacks: [[Bitboard; 64]; 2],
    pub knight_attacks: [Bitboard; 64],
    pub king_attacks: [Bitboard; 64],
}

#[derive(Clone)]
pub struct Magic<const N: usize> {
    pub magic_num: u64,
    pub direction_mask: Bitboard, 
    pub attack_table: Vec<Option<Bitboard>>, 
}

impl<const N: usize> Magic<N> {
    pub fn new_magic(magic_num: u64, direction_mask: Bitboard, attack_table: Vec<Option<Bitboard>>) -> Magic<N> {
        Magic {
            magic_num,
            direction_mask,
            attack_table,
        }
    }
}

impl<const N: usize> Default for Magic<N> {
    fn default() -> Self {
        Magic {
            magic_num: 0,
            direction_mask: 0,
            attack_table: vec![None; N],
        }
    }
}

pub type BishopMagic = Magic<{ 1 << BISHOP_INDEX_BITS }>;
pub type RookMagic   = Magic<{ 1 << ROOK_INDEX_BITS }>;

pub enum MagicError {
    CollisionDetected(usize),
}
