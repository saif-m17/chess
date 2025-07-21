// Script for checking the difference between stockfish moves generated + this versions
// To use, create directory test_move_data in move_gen + text files my_moves.txt and 
// stockfish_moves.txt with moves written on each line of the form "{mv}: {# moves from branch}"

use std::collections::HashMap;
use std::fs;


fn main() {
    println!("CWD: {}", std::env::current_dir().unwrap().display());

    let my_moves_string = fs::read_to_string("../test_move_data/my_moves.txt").unwrap(); 
    let my_moves: HashMap<String, u64> = my_moves_string
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() != 2 {
                    return None;
                }
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().parse::<u64>().ok()?;
                Some((key, value))
            })
            .collect();

    println!("reached here"); 
    
    let stockfish_moves_string = fs::read_to_string("../test_move_data/stockfish_moves.txt").unwrap(); 
    let stockfish_moves: HashMap<String, u64> = stockfish_moves_string
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() != 2 {
                    return None;
                }
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().parse::<u64>().ok()?;
                Some((key, value))
            })
            .collect();

    compare_maps(&my_moves, &stockfish_moves);
}


fn compare_maps(
    my_moves: &HashMap<String, u64>,
    stockfish_moves: &HashMap<String, u64>,
) {
    for (mv, &count) in stockfish_moves {
        match my_moves.get(mv) {
            Some(&my_count) if my_count != count => {
                println!("Different count for {}: mine = {}, stockfish = {}", mv, my_count, count);
            }
            None => {
                println!("Missing in my moves: {} (Stockfish: {})", mv, count);
            }
            _ => {} // same
        }
    }

    for (mv, &count) in my_moves {
        if !stockfish_moves.contains_key(mv) {
            println!("Extra move in my output: {} (Mine: {})", mv, count);
        }
    }
}