use chess_core::board::Board;
use chess_core::moves::{*}; 
use chess_core::utils::{*}; 
use std::time::Instant;

fn main() {

    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    for d in 1..=7 {
        let now = Instant::now();
        let n = perft(&mut board, d, Color::White);
        println!("Depth {d}: {n} nodes in {:?}", now.elapsed());
    }

}