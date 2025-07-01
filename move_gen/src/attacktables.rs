use crate::bitboards::{*};
use crate::bitboards::Bitboard;
use crate::moves::Square; 

pub struct AttackTables {
    pub pawn_attacks: [[Bitboard; 64]; 2],
    pub knight_attacks: [Bitboard; 64],
    pub king_attacks: [Bitboard; 64],
}

impl AttackTables {
    pub fn new() -> Self {
        let mut pawn_attacks = [[0u64; 64]; 2]; 
        let mut knight_attacks = [0u64; 64];
        let mut king_attacks = [0u64; 64];

        for square in 0..64 {
            let bb = 1u64 << square;
            pawn_attacks[0][square] = PAWN_ATTACK_LEFT[0](bb) | PAWN_ATTACK_RIGHT[0](bb);
            pawn_attacks[1][square] = PAWN_ATTACK_LEFT[1](bb) | PAWN_ATTACK_RIGHT[1](bb);
            knight_attacks[square] = 
        } 

        AttackTables { 
            pawn_attacks,
            knight_attacks,
            king_attacks,
        }
    }
}

