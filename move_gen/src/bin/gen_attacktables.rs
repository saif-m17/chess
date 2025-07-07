use std::fs::File;
use std::io::Write;

use chess_core::bitboards::{*}; 

fn main() {
    let attacks = AttackTables {
        pawn_attacks: generate_pawn_attacks(),
        knight_attacks: generate_knight_attacks(),
        king_attacks: generate_king_attacks(),
    }; 
    write_attacks_to_file(attacks);
}

pub struct AttackTables {
    pub pawn_attacks: [[Bitboard; 64]; 2],
    pub knight_attacks: [Bitboard; 64],
    pub king_attacks: [Bitboard; 64],
}

/// Generate tables at compile time
fn generate_pawn_attacks() -> [[Bitboard; 64]; 2] {
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

fn generate_king_attacks() -> [Bitboard; 64] {
    let mut king_attacks = [0u64; 64];
    let mut square = 0;
    
    while square < 64 {
        let bb: Bitboard = 1u64 << square;
        king_attacks[square] = gen_king_attacks(bb);
        square += 1;
    }
    
    king_attacks
}

fn generate_knight_attacks() -> [Bitboard; 64] {
    let mut knight_attacks = [0u64; 64];
    let mut square = 0;
    
    while square < 64 {
        let bb: Bitboard = 1u64 << square;
        knight_attacks[square] = gen_knight_attacks(bb);
        square += 1;
    }
    
    knight_attacks
}

fn write_attacks_to_file(attacks: AttackTables) {
    let path = "src/attacktables/basic_attacktables.rs"; 
    let mut file = File::create(path).unwrap(); 

    writeln!(file, "use crate::bitboards::Bitboard;").unwrap();
    writeln!(file, "use crate::attacktables::AttackTables;").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "pub static ATTACK_TABLES: AttackTables = AttackTables {{").unwrap();
    
    // Write pawn attacks
    writeln!(file, "    pawn_attacks: [").unwrap();
    for color in 0..2 {
        writeln!(file, "        [").unwrap();
        for square in 0..64 {
            writeln!(file, "            0x{:016x},", attacks.pawn_attacks[color][square]).unwrap();
        }
        writeln!(file, "        ],").unwrap();
    }
    writeln!(file, "    ],").unwrap();
    
    // Write knight attacks
    writeln!(file, "    knight_attacks: [").unwrap();
    for square in 0..64 {
        writeln!(file, "        0x{:016x},", attacks.knight_attacks[square]).unwrap();
    }
    writeln!(file, "    ],").unwrap();
    
    // Write king attacks
    writeln!(file, "    king_attacks: [").unwrap();
    for square in 0..64 {
        writeln!(file, "        0x{:016x},", attacks.king_attacks[square]).unwrap();
    }
    writeln!(file, "    ],").unwrap();
    
    writeln!(file, "}};").unwrap();
    
    println!("Attack tables written to {}", path);
}