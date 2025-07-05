use crate::bitboards::Bitboard;
//use crate::moves::{Direction, Square};  

const BISHOP_INDEX_BITS: usize = 9;
const ROOK_INDEX_BITS: usize = 12;
struct Magic<const N: usize> {
    magic_num: u64,
    direction_mask: Bitboard, 
    attack_table: [Bitboard; N], 
}

impl<const N: usize> Magic<N> {
    pub fn new_magic(magic_num: u64, direction_mask: Bitboard, attack_table: [Bitboard; N]) -> Magic<N> {
        Magic {
            magic_num,
            direction_mask,
            attack_table,
        }
    }
}

type BishopMagic = Magic<{ BISHOP_INDEX_BITS }>;
type RookMagic   = Magic<{ ROOK_INDEX_BITS }>;

// TODO 
// static BISHOP_MAGICS: [BishopMagic; 64] = [...];
// static ROOK_MAGICS:   [RookMagic; 64]   = [...];

// pub fn gen_magics()

// pub fn bishop_attacks(occupied: Bitboard, square: Square) -> Bitboard {
//     todo!()
// }


