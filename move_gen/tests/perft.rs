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
fn test_starting_position_depth6() {
    run_perft_test("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 6, 119060324);
}

#[test]
fn test_starting_position_depth7() {
    run_perft_test("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 7, 3195901860);
}

#[test]
fn test_kiwipete_depth2() {
    run_perft_test("r3kq1r/p1pp1pb1/bn2pQp1/3PN3/1p2P3/2N4p/PPPBBPPP/R3K2R w KQkq - 1 2", 2, 1693);
}

#[test]
fn test_kiwipete_depth5() {
    run_perft_test("r3kq1r/p1pp1pb1/bn2pQp1/3PN3/1p2P3/2N4p/PPPBBPPP/R3K2R w KQkq - 1 2", 5, 144415532);
}

#[test]
fn test_chess_pw_position3() {
    run_perft_test("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 2, 191);
}

#[test]
fn test_chess_pw_position3_b4c4() {
    run_perft_test("8/2p5/3p4/KP5r/2R2p1k/8/4P1P1/8 b - - 1 1", 1, 15);
}


#[test]
fn test_chess_pw_position3_2() {
    run_perft_test("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 6, 11030083);
}