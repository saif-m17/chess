use crate::bitboards::{*};
use crate::bitboards::Bitboard;
use crate::moves::Square; 

pub struct AttackTables {
    pub pawn_attacks: [[Bitboard; 64]; 2],
    pub knight_attacks: [Bitboard; 64],
    pub king_attacks: [Bitboard; 64],
}

/// Generate tables at compile time
const fn generate_pawn_attacks() -> [[Bitboard; 64]; 2] {
    let mut pawn_attacks = [[0u64; 64]; 2];
    let mut square = 0;
    
    while square < 64 {
        let bb: Bitboard = 1u64 << square;
        pawn_attacks[0][square] = pawn_attack_left_white(bb) | pawn_attack_right_white(bb);
        pawn_attacks[1][square] = pawn_attack_left_black(bb) | pawn_attack_right_black(bb);
        square += 1;
    }
    
    pawn_attacks
}

const fn generate_king_attacks() -> [Bitboard; 64] {
    let mut king_attacks = [0u64; 64];
    let mut square = 0;
    
    while square < 64 {
        let bb: Bitboard = 1u64 << square;
        king_attacks[square] = gen_king_attacks(bb);
        square += 1;
    }
    
    king_attacks
}

const fn generate_knight_attacks() -> [Bitboard; 64] {
    let mut knight_attacks = [0u64; 64];
    let mut square = 0;
    
    while square < 64 {
        let bb: Bitboard = 1u64 << square;
        knight_attacks[square] = gen_knight_attacks(bb);
        square += 1;
    }
    
    knight_attacks
}

// Static tables generated at compile time
static ATTACK_TABLES: AttackTables = AttackTables {
    pawn_attacks: generate_pawn_attacks(),
    knight_attacks: generate_knight_attacks(),
    king_attacks: generate_king_attacks(),
};

impl AttackTables {
    // Return reference to static tables
    pub fn get() -> &'static AttackTables {
        &ATTACK_TABLES
    }

}

