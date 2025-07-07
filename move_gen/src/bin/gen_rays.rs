use std::fs::File;
use std::io::Write;
use chess_core::moves::DIRECTION_COORDINATES;

use chess_core::bitboards::{Bitboard, BitboardExt}; 
use chess_core::moves::Square; 

fn main() {
    let mut rays = [[0u64; 8]; 64];
    let mut rays_without_end = [[0u64; 8]; 64]; 

    for square in 0..64 {
        for dir in 0..8 {
            rays[square][dir] = compute_ray_include_edge(square as u8, dir as u8);
            rays_without_end[square][dir] = compute_ray(square as u8, dir as u8); 
        }
    }

    write_rays_to_file(&rays);
}

fn compute_ray(square: u8, dir: u8) -> Bitboard {
    let mut result = 0u64;
    let mut current = Square::try_from(square as u64).unwrap();

    while let Some(next) = step_in_direction(current as u8, dir) {
        if step_in_direction(next as u8, dir).is_some() {
            result |= Bitboard::from_square(next);
        }
        current = next;
    }

    result
}

fn compute_ray_include_edge(square: u8, dir: u8) -> Bitboard {
    let mut result = 0u64;
    let mut current = Square::try_from(square as u64).unwrap();

    while let Some(next) = step_in_direction(current as u8, dir) {
        result |= Bitboard::from_square(next);
        current = next;
    }

    result

}


fn step_in_direction(square: u8, dir: u8) -> Option<Square> {
    let idx = square as i8; 
    let rank = idx / 8;
    let file = idx % 8; 

    let (dr, df) = DIRECTION_COORDINATES[dir as usize];
    let new_rank = rank + dr;
    let new_file = file + df; 

    if (0..8).contains(&new_rank) && (0..8).contains(&new_file) {
        let new_index = (new_rank * 8 + new_file) as u8;
        Square::try_from(new_index as u64).ok()
    } else {
        None
    } 

}

fn write_rays_to_file(rays: &[[u64; 8]; 64]) {
    let path = "src/attacktables/rays_table_to_edge.rs"; // changed for including edge of board
    let mut file = File::create(path).unwrap();

    writeln!(file, "use crate::bitboards::Bitboard;").unwrap();
    writeln!(file, "pub static RAYS_WITH_EDGE: [[Bitboard; 8]; 64] = [").unwrap(); // change to inclde edge of board 

    for row in rays {
        write!(file, "    [").unwrap();
        for &bb in row {
            write!(file, "0x{:016x}, ", bb).unwrap();
        }
        writeln!(file, "],").unwrap();
    }

    writeln!(file, "];").unwrap();

    println!("Wrote rays table to `{}`", path);
}
