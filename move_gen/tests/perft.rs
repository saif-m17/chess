use chess_core::board::Board;
use chess_core::movegen::{*};
use chess_core::moves::{*}; 
use chess_core::utils::{*}; 

fn run_perft_test(fen: &str, depth: u64, expected: u64) {
    let mut board = Board::new(); // bug in from fen for now
    let color = if fen.contains(" w ") { Color::White } else { Color::Black };
    let result = divide(&mut board, depth, color);
    assert_eq!(result, expected, "Failed at depth {} for FEN {}", depth, fen);
}

#[test]
fn test_starting_position_depth1() {
    run_perft_test("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 1, 20);
}

#[test]
fn test_starting_position_depth2() {
    run_perft_test("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 2, 400);
}

#[test]
fn test_starting_position_depth3() {
    run_perft_test("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 3, 8902);
}

#[test]
fn test_starting_position_depth4() {
    run_perft_test("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 4, 197281);
}

#[test]
fn test_starting_position_depth5() {
    run_perft_test("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 5, 4865609);
}

#[test]
fn test_kiwipete_depth2() {
    run_perft_test("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", 2, 2039);
}