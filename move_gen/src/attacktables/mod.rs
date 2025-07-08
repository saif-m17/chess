use crate::bitboards::Bitboard;

mod rays_table;
pub use rays_table::RAYS;

mod rays_table_to_edge;
pub use rays_table_to_edge::RAYS_WITH_EDGE; 

mod basic_attacktables;
pub use basic_attacktables::ATTACK_TABLES;

mod magic_tables;
pub use magic_tables::BISHOP_MAGICS;
pub use magic_tables::ROOK_MAGICS; 

mod in_between_squares;
pub use in_between_squares::IN_BETWEEN_SQUARES; 

pub const BISHOP_INDEX_BITS: usize = 9;
pub const ROOK_INDEX_BITS: usize = 12;

pub struct AttackTables {
    pub pawn_attacks: [[Bitboard; 64]; 2],
    pub knight_attacks: [Bitboard; 64],
    pub king_attacks: [Bitboard; 64],
}

#[derive(Clone)]
pub struct Magic {
    pub magic_num: u64,
    pub direction_mask: Bitboard,
    pub index_bits: usize, 
    pub attack_table: Vec<Option<Bitboard>>, 
}

impl Magic {
    pub fn new_magic(magic_num: u64, direction_mask: Bitboard, index_bits: usize, attack_table: Vec<Option<Bitboard>>) -> Magic {
        Magic {
            magic_num,
            direction_mask,
            index_bits,
            attack_table,
        }
    }

    pub fn default(index_bits: usize) -> Self {
        Magic {
            magic_num: 0,
            direction_mask: 0,
            index_bits,
            attack_table: vec![None; 1 << index_bits],
        }
    }
}

pub enum MagicError {
    CollisionDetected(usize),
}
