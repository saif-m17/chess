use chess_core::attacktables::RAYS_WITH_EDGE;
use chess_core::bitboards::{Bitboard, BitboardExt}; 

use std::fs::File;
use std::io::Write;

fn main() {
    let in_between_squares = compute_between_squares(&RAYS_WITH_EDGE); 
    write_in_between_to_file(&in_between_squares);
}


fn compute_between_squares(rays: &[[Bitboard; 8]; 64]) -> [[Bitboard; 64]; 64] {
    let mut in_between = [[0; 64]; 64];

    for from in 0..64 {
        for to in 0..64 {
            if from == to { continue; }

            for direction in 0..8 {
                let ray = rays[from][direction]; 
                if ray.get_bit(to.try_into().unwrap()) {
                    let ray_from_to = rays[to][direction]; 
                    
                    let mask_below_to = ray & !ray_from_to; 
                    let bet = mask_below_to & !(1u64 << to); 

                    in_between[from][to] = bet; 
                    break; 
                }
            }
        }
    }

    in_between
}

fn write_in_between_to_file(in_between: &[[Bitboard; 64]; 64]) {
    let path = "src/attacktables/in_between_squares.rs"; 
    let mut file = File::create(path).unwrap();

    writeln!(file, "use crate::bitboards::Bitboard;").unwrap();
    writeln!(file, "pub static IN_BETWEEN_SQUARES: [[Bitboard; 64]; 64] = [").unwrap(); 

    for row in in_between {
        write!(file, "    [").unwrap();
        for &bb in row {
            write!(file, "0x{:016x}, ", bb).unwrap();
        }
        writeln!(file, "],").unwrap();
    }

    writeln!(file, "];").unwrap();

    println!("Wrote in_between_squares table to `{}`", path);
}