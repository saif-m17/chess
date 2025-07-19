use chess_core::board::Board;
use chess_core::movegen::{*};
use chess_core::moves::{*}; 
use chess_core::utils::{*}; 

fn run_perft_test(fen: &str, depth: u64, expected: u64) {
    let mut board = Board::from_fen(fen).unwrap();
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
fn test_from_h2h3_depth2() {
    run_perft_test("rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b KQkq - 0 1", 2, 380);
}

#[test]
fn test_from_h7h5_depth1() {
    run_perft_test("rnbqkbnr/1ppppppp/8/p7/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 2", 1, 19);
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

#[test]
fn test_kiwipete_after_a2a3() {
    run_perft_test("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/P1N2Q1p/1PPBBPPP/R3K2R b KQkq - 0 1", 1, 44);
}

#[test]
fn test_kiwipete_after_queenside_castle() {
    run_perft_test("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/2KR3R b kq - 1 1", 1, 43);
}

#[test]
fn test_kiwipete_after_a2a4() {
    run_perft_test("r3k2r/p1ppqpb1/bn2pnp1/3PN3/Pp2P3/2N2Q1p/1PPBBPPP/R3K2R b KQkq a3 0 1", 1, 44);
}