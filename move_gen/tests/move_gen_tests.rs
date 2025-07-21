use chess_core::attacktables::ROOK_MAGICS;
use chess_core::movegen::{*};
use chess_core::bitboards::{*}; 
use chess_core::moves::{*, Color::*, Piece::*}; 
use chess_core::board::Board;


// Fen parsing checks
#[test]
fn test_valid_startpos_fen() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen).expect("Failed to parse startpos FEN");

    // Check white pawns
    let white_pawns = board.pieces[White as usize][Pawn as usize];
    assert_eq!(white_pawns, 0x000000000000FF00);

    // Check black pawns
    let black_pawns = board.pieces[Black as usize][Pawn as usize];
    assert_eq!(black_pawns, 0x00FF000000000000);

    // Check castling rights
    assert_eq!(board.move_changed_castling_rights[White as usize][0], -1); // Q
    assert_eq!(board.move_changed_castling_rights[White as usize][1], -1); // K
    assert_eq!(board.move_changed_castling_rights[Black as usize][0], -1); // q
    assert_eq!(board.move_changed_castling_rights[Black as usize][1], -1); // k

    // Check en passant
    assert_eq!(board.en_passant_square, None);

    // Check side
    assert_eq!(board.side, White);

    // Check move number and half-move clock
    assert_eq!(board.move_number, 1);
    assert_eq!(board.half_move_clock, 0);
}

#[test]
fn test_invalid_format_fen() {
    let bad_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -"; // missing fields
    let result = Board::from_fen(bad_fen);
    assert!(result.is_err());
    assert!(matches!(result, Err(FenError::InvalidFormat(_))));
}

#[test]
fn test_invalid_piece() {
    let bad_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPXPPP/RNBQKBNR w KQkq - 0 1";
    let result = Board::from_fen(bad_fen);
    assert!(matches!(result, Err(FenError::InvalidPiece('X'))));
}

#[test]
fn test_valid_custom_fen_with_en_passant() {
    let fen = "8/8/8/3pP3/8/8/8/8 w - d6 0 2";
    let board = Board::from_fen(fen).unwrap();

    assert_eq!(board.en_passant_square, Some(Square::try_from(43).unwrap())); // d6 -> index 35
    assert_eq!(board.side, White);
    assert_eq!(board.move_number, 2);
    assert_eq!(board.half_move_clock, 0);
}

#[test]
fn test_valid_kiwipete_fen() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let board = Board::from_fen(fen).expect("Failed to parse startpos FEN");

    board.display();

    // Check castling rights
    assert_eq!(board.move_changed_castling_rights[White as usize][0], -1); // Q
    assert_eq!(board.move_changed_castling_rights[White as usize][1], -1); // K
    assert_eq!(board.move_changed_castling_rights[Black as usize][0], -1); // q
    assert_eq!(board.move_changed_castling_rights[Black as usize][1], -1); // k
}

#[test]
fn test_rook_edge_sliding_attacks_gen() {
    let fen = "r1bqkb1r/pp3ppp/2nppn2/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQR1K1 b kq - 1 6";
    let board = Board::from_fen(fen).expect("Failed to parse position");

    let attacks = get_sliding_piece_attacks(board.get_all_pieces(), Square::E1 , &ROOK_MAGICS); 

    attacks.display();

}

#[test]
fn test_chess_pw_position3_b3b4_moves() {
    let board = Board::from_fen("8/2p5/3p4/KP5r/2R2p1k/8/4P1P1/8 b - - 1 1").unwrap();
    let mut moves: Vec<Move> = Vec::new();
    get_legal_moves(&board, Black, &mut moves);

    for mv in &moves {
        println!("{mv}"); 
    }

    println!("{}", moves.len()); 

}