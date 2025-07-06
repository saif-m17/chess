use std::fs::File;
use std::io::Write;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use chess_core::attacktables::{Magic, MagicError, RAYS};
use chess_core::bitboards::{*}; 
use chess_core::moves::Direction;


const ROOK_DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
const BISHOP_DIRECTIONS: [Direction; 4] = [Direction::NorthEast, Direction::NorthWest, Direction::SouthEast, Direction::SouthWest]; 

fn main() {
    let bishop_magics = get_magics(BISHOP_DIRECTIONS);
    let rook_magics = get_magics(ROOK_DIRECTIONS);
    write_magics_to_file(rook_magics, bishop_magics);
}

fn get_magics(directions_list: [Direction; 4]) -> Vec<Magic> {
    let mut magics: Vec<Magic> = vec![Magic::default(1); 64]; 
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
                magics[square as usize] = Magic::new_magic(candidate_magic, rays, index_bits, attack_table); 
                break 
            }
            attempt_count += 1;
        } 
    }
    magics
}

fn check_table(candidate_magic: u64, rays: Bitboard, directions_list: [Direction; 4], square: u8, index_bits: usize) -> Result<Vec<Option<Bitboard>>, MagicError>   {
    let mut attack_table: Vec<Option<Bitboard>> =  vec![None; 1 << index_bits];
    let mut blockers = rays; 

    loop {
        let magic_index = blockers.wrapping_mul(candidate_magic) >> (64 - index_bits);
        if attack_table[magic_index as usize].is_some() {
            return Err(MagicError::CollisionDetected(magic_index as usize))
        } else {
            let attacks = get_attacks(directions_list, blockers, square);
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

fn write_magics_to_file(rook_magics: Vec<Magic>, bishop_magics: Vec<Magic>) {
    let path = "src/attacktables/magic_tables.rs";
    let mut file = File::create(path).unwrap();

    writeln!(file, "use crate::bitboards::Bitboard;").unwrap();
    writeln!(file, "use crate::attacktables::Magic;").unwrap();
    writeln!(file, "").unwrap();

    // Write rook magics
    writeln!(file, "pub static ROOK_MAGICS: [Magic; 64] = [").unwrap();
    for (i, magic) in rook_magics.iter().enumerate() {
        writeln!(file, "    Magic {{").unwrap();
        writeln!(file, "        magic_num: 0x{:016x},", magic.magic_num).unwrap();
        writeln!(file, "        direction_mask: 0x{:016x},", magic.direction_mask).unwrap();
        writeln!(file, "        index_bits: {},", magic.index_bits).unwrap();
        writeln!(file, "        attack_table: [").unwrap();

        for attack in &magic.attack_table {
            match attack {
                Some(bitboard) => writeln!(file, "            Some(0x{:016x}),", bitboard).unwrap(),
                None => writeln!(file, "            None,").unwrap(),
            }
        }

        writeln!(file, "        ],").unwrap();
        writeln!(file, "    }},").unwrap();
    }
    writeln!(file, "];").unwrap();
    writeln!(file).unwrap();

    // Write bishop magics
    writeln!(file, "pub static BISHOP_MAGICS: [Magic; 64] = [").unwrap();
    for (i, magic) in bishop_magics.iter().enumerate() {
        writeln!(file, "    Magic {{").unwrap();
        writeln!(file, "        magic_num: 0x{:016x},", magic.magic_num).unwrap();
        writeln!(file, "        direction_mask: 0x{:016x},", magic.direction_mask).unwrap();
        writeln!(file, "        index_bits: {},", magic.index_bits).unwrap();
        writeln!(file, "        attack_table: [").unwrap();

        for attack in &magic.attack_table {
            match attack {
                Some(bitboard) => writeln!(file, "            Some(0x{:016x}),", bitboard).unwrap(),
                None => writeln!(file, "            None,").unwrap(),
            }
        }

        writeln!(file, "        ],").unwrap();
        writeln!(file, "    }},").unwrap();
    }
    writeln!(file, "];").unwrap();

    println!("Magic tables written to {}", path);
}