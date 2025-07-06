use std::fs::File;
use std::io::Write;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use chess_core::attacktables::{RookMagic, BishopMagic, MagicError, RAYS, BISHOP_INDEX_BITS, ROOK_INDEX_BITS};
use chess_core::bitboards::{*}; 
use chess_core::moves::Direction;


const ROOK_DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
const BISHOP_DIRECTIONS: [Direction; 4] = [Direction::NorthEast, Direction::NorthWest, Direction::SouthEast, Direction::SouthWest]; 

fn main() {
    let bishop_magics = get_bishop_magics();
    let rook_magics = get_rook_magics();
    write_magics_to_file(rook_magics, bishop_magics);
}

fn get_rook_magics() -> Vec<Magic> {
    let directions_list = ROOK_DIRECTIONS;
    let mut magics: Vec<Magic> = vec![Magic::default(); 64]; 
    let mut rng = StdRng::seed_from_u64(300);

    for square in 0u8..64 {
        let directions = get_directions_bb(square, directions_list);
        let rays = directions[0] | directions[1] | directions[2] | directions[3]; 
        let index_bits = count_relevant_bits(rays); 
        let mut attempt_count = 1; 
        loop {
            let candidate_magic = if attempt_count % 1000 == 0 {
                rng.r#gen::<u64>() & rng.r#gen::<u64>() & rng.r#gen::<u64>()
            } else {
                rng.r#gen::<u64>() & rng.r#gen::<u64>()
            };
            if let Ok(attack_table) = check_table(candidate_magic, rays, directions_list, square, index_bits) {
                magics[square as usize] = RookMagic::new_magic(candidate_magic, rays, attack_table); 
                break 
            }
            attempt_count += 1;
        } 
    }
    magics
}

fn get_bishop_magics() -> Vec<BishopMagic> {
    let directions_list = BISHOP_DIRECTIONS;
    let mut magics: Vec<BishopMagic> = vec![BishopMagic::default(); 64];
    let mut rng = StdRng::seed_from_u64(227);
    for square in 0u8..64 {
        // println!("Starting square {}", square);
        let directions = get_directions_bb(square, directions_list);
        // println!("Square {}: NE={:016x}, NW={:016x}, SE={:016x}, SW={:016x}", 
        // square, directions[0], directions[1], directions[2], directions[3]);
        let rays = directions[0] | directions[1] | directions[2] | directions[3];
        // println!("Rays = {rays}"); 
        let mut attempt_count = 1; 
        loop {
            let candidate_magic = if attempt_count % 1000 == 0 {
                rng.r#gen::<u64>() & rng.r#gen::<u64>() & rng.r#gen::<u64>()
            } else {
                rng.r#gen::<u64>() & rng.r#gen::<u64>()
            };
            if let Ok(attack_table) = check_table(candidate_magic, rays, directions_list, square, BISHOP_INDEX_BITS) {
                magics[square as usize] = BishopMagic::new_magic(candidate_magic, rays, attack_table); 
                break
            }
            attempt_count += 1;
        }
    }

    magics
}

fn check_table(candidate_magic: u64, rays: Bitboard, directions_list: [Direction; 4], square: u8, index_bits: usize) -> Result<Vec<Option<Bitboard>>, MagicError>   {
    let index_bits2 = count_relevant_bits(rays);
    let mut attack_table: Vec<Option<Bitboard>> =  vec![None; 1 << index_bits2];
    let mut blockers = rays; 

    loop {
        let magic_index = blockers.wrapping_mul(candidate_magic) >> (64 - index_bits2);
        if attack_table[magic_index as usize].is_some() {
            return Err(MagicError::CollisionDetected(magic_index as usize))
        } else {
            let attacks = get_attacks(directions_list, blockers, square);
            // println!("Square {}, blockers: 0x{:016x}, attacks: 0x{:016x}", square, blockers, attacks); 
            attack_table[magic_index as usize] = Some(attacks); 
        }
        if blockers == 0 {
            break; 
        } 
        blockers = blockers.wrapping_sub(1) & rays;
    }
    Ok(attack_table)

}


fn get_directions_bb(square: u8, directions_list: [Direction; 4]) -> [Bitboard; 4] {
    [RAYS[square as usize][directions_list[0] as usize],
    RAYS[square as usize][directions_list[1] as usize],
    RAYS[square as usize][directions_list[2] as usize],
    RAYS[square as usize][directions_list[3] as usize],]
}

fn get_attacks(directions: [Direction; 4], blockers: Bitboard, square: u8) -> Bitboard {
    let mut attacks = 0u64; 
    for direction in directions {
        let direction_ray = RAYS[square as usize][direction as usize];
        let relevant_blockers = direction_ray & blockers; 
        let first_blocker = if relevant_blockers != 0 {
            match direction {
                Direction::North => relevant_blockers.trailing_zeros(),
                Direction::NorthEast => relevant_blockers.trailing_zeros(),
                Direction::NorthWest => relevant_blockers.trailing_zeros(),
                Direction::West => relevant_blockers.trailing_zeros(),
                Direction::East => 63 - relevant_blockers.leading_zeros(),
                Direction::South => 63 - relevant_blockers.leading_zeros(),
                Direction::SouthEast => 63 - relevant_blockers.leading_zeros(),
                Direction::SouthWest => 63 - relevant_blockers.leading_zeros(),
            }
        } else {
            64
        }; 
        let attack_this_direction = if first_blocker < 64 {
            let first_blocker_ray = RAYS[first_blocker as usize][direction as usize];
            first_blocker_ray ^ direction_ray 
        } else {
            direction_ray
        }; 

        attacks |= attack_this_direction; 
    }
    attacks
}

fn count_relevant_bits(rays: Bitboard) -> usize {
    rays.count_ones() as usize
}

fn write_magics_to_file(rook_magics: Vec<RookMagic>, bishop_magics: Vec<BishopMagic>) {
    let path = "src/attacktables/magic_tables.rs";
    let mut file = File::create(path).unwrap();

    writeln!(file, "use crate::bitboards::Bitboard;").unwrap();
    writeln!(file, "use crate::attacktables::{{RookMagic, BishopMagic}};").unwrap();
    writeln!(file, "").unwrap();

    // Write rook magics
    writeln!(file, "pub static ROOK_MAGICS: [RookMagic; 64] = [").unwrap();
    for (i, magic) in rook_magics.iter().enumerate() {
        writeln!(file, "    RookMagic {{").unwrap();
        writeln!(file, "        magic_num: 0x{:016x},", magic.magic_num).unwrap();
        writeln!(file, "        direction_mask: 0x{:016x},", magic.direction_mask).unwrap();
        writeln!(file, "        attack_table: [").unwrap();
        
        for attack in &magic.attack_table {
            match attack {
                Some(bitboard) => writeln!(file, "            Some(0x{:016x}),", bitboard).unwrap(),
                None => writeln!(file, "            None,").unwrap(),
            }
        }
        
        writeln!(file, "        ],").unwrap();
        if i < 63 {
            writeln!(file, "    }},").unwrap();
        } else {
            writeln!(file, "    }}").unwrap();
        }
    }
    writeln!(file, "];").unwrap();
    writeln!(file, "").unwrap();

    // Write bishop magics
    writeln!(file, "pub static BISHOP_MAGICS: [BishopMagic; 64] = [").unwrap();
    for (i, magic) in bishop_magics.iter().enumerate() {
        writeln!(file, "    BishopMagic {{").unwrap();
        writeln!(file, "        magic_num: 0x{:016x},", magic.magic_num).unwrap();
        writeln!(file, "        direction_mask: 0x{:016x},", magic.direction_mask).unwrap();
        writeln!(file, "        attack_table: [").unwrap();
        
        for attack in &magic.attack_table {
            match attack {
                Some(bitboard) => writeln!(file, "            Some(0x{:016x}),", bitboard).unwrap(),
                None => writeln!(file, "            None,").unwrap(),
            }
        }
        
        writeln!(file, "        ],").unwrap();
        if i < 63 {
            writeln!(file, "    }},").unwrap();
        } else {
            writeln!(file, "    }}").unwrap();
        }
    }
    writeln!(file, "];").unwrap();

    println!("Magic tables written to {}", path);

}