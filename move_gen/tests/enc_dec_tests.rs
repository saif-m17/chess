use chess_core::game_state::GameState; 
use chess_core::tensor::{TensorBuffer, FeatureSchema};
use chess_core::action_space::{encode_action, decode_action};

#[test]
fn encode_action_test() {
    let mut game = GameState::new();
    let move_count = game.get_gamestate_legal_moves().len(); 

    for i in 0..move_count {
        let mv = &game.get_gamestate_legal_moves().get(i).expect("size checked."); 
        let encoding = encode_action(mv);
        let intent = decode_action(encoding).expect("move should be legal.");
        let decoded_move = game.realize_move(intent).expect("move decoding should work."); 
        assert_eq!(*mv, decoded_move, "Move: {:?} did not roundtrip correctly", mv); 
    }
}

#[test]
fn encode_action_test_kiwipete() {
    let mut game = GameState::from_fen("r3kq1r/p1pp1pb1/bn2pQp1/3PN3/1p2P3/2N4p/PPPBBPPP/R3K2R w KQkq - 1 2").expect("valid fen."); 
    let move_count = game.get_gamestate_legal_moves().len(); 

    for i in 0..move_count {
        let mv = &game.get_gamestate_legal_moves().get(i).expect("size checked"); 
        let encoding = encode_action(mv);
        let intent = decode_action(encoding).expect("move should be legal.");
        let decoded_move = game.realize_move(intent).expect("move decoding should work."); 
        assert_eq!(*mv, decoded_move, "Move: {:?} did not roundtrip correctly", mv); 
    }
}

#[test]
fn encode_action_test_position3cpw() {
    let mut game = GameState::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").expect("valid fen."); 
    let move_count = game.get_gamestate_legal_moves().len(); 

    for i in 0..move_count {
        let mv = &game.get_gamestate_legal_moves().get(i).expect("size checked"); 
        let encoding = encode_action(mv);
        let intent = decode_action(encoding).expect("move should be legal.");
        let decoded_move = game.realize_move(intent).expect("move decoding should work."); 
        assert_eq!(*mv, decoded_move, "Move: {:?} did not roundtrip correctly", mv); 
    }
}

#[test]
fn encode_action_test_promocheck() {
    let mut game = GameState::from_fen("8/8/3p4/KP5r/1R3p1k/8/2p1P1P1/8 b - - 1 5").expect("valid fen."); 
    let move_count = game.get_gamestate_legal_moves().len(); 

    for i in 0..move_count {
        let mv = &game.get_gamestate_legal_moves().get(i).expect("size checked");
        let encoding = encode_action(mv);
        let intent = decode_action(encoding).expect("move should be legal.");
        let decoded_move = game.realize_move(intent).expect("move decoding should work."); 
        assert_eq!(*mv, decoded_move, "Move: {:?} did not roundtrip correctly", mv); 
    }
}

#[test]
fn tensorbuffer_startpos_encoding() {
    let game = GameState::new();
    let schema = FeatureSchema::new();
    let mut buffer = TensorBuffer::new(schema);

    buffer.write_from(&game);

    // Basic shape checks
    assert_eq!(buffer.len(), 20 * 64, "Tensor buffer should have 20 planes of size 64");

    // === Piece placement tests ===
    // White pawns on rank 2
    let wp_idx = buffer.schema().index_of("white_pawns").unwrap();
    for file in 0..8 {
        let sq = 1 * 8 + file; 
        assert_eq!(buffer.as_slice()[wp_idx * 64 + sq], 1.0, "White pawn missing at file {}", file);
    }

    // Black pawns on rank 7
    let bp_idx = buffer.schema().index_of("black_pawns").unwrap();
    for file in 0..8 {
        let sq = 6 * 8 + file; 
        assert_eq!(buffer.as_slice()[bp_idx * 64 + sq], 1.0, "Black pawn missing at file {}", file);
    }

    // White knights at b1 (sq=1) and g1 (sq=6)
    let wn_idx = buffer.schema().index_of("white_knights").unwrap();
    for &sq in &[1, 6] {
        assert_eq!(buffer.as_slice()[wn_idx * 64 + sq], 1.0, "White knight missing at square {}", sq);
    }

    // Black king at e8 (sq=60)
    let bk_idx = buffer.schema().index_of("black_king").unwrap();
    assert_eq!(buffer.as_slice()[bk_idx * 64 + 60], 1.0, "Black king missing at e8");

    // === Metadata planes ===
    // Side to move (white at start → EMPTY board)
    let stm_idx = buffer.schema().index_of("side_to_move").unwrap();
    assert!(buffer.as_slice()[stm_idx * 64..stm_idx * 64 + 64]
        .iter()
        .all(|&x| (x - 0.0).abs() < 1e-6),
        "Side-to-move plane should be all 1s for white");

    // Castling rights at start (all true → FULL board)
    for &castle in &["white_queenside", "white_kingside", "black_queenside", "black_kingside"] {
        let cidx = buffer.schema().index_of(castle).unwrap();
        assert!(buffer.as_slice()[cidx * 64..cidx * 64 + 64]
            .iter()
            .all(|&x| (x - 1.0).abs() < 1e-6),
            "Castling plane {} should be all 1s", castle);
    }

    // En passant square should be empty at start
    let ep_idx = buffer.schema().index_of("en_passant_sq").unwrap();
    assert!(buffer.as_slice()[ep_idx * 64..ep_idx * 64 + 64]
        .iter()
        .all(|&x| x == 0.0),
        "En passant plane should be all 0s at start");

    // Halfmove clock = 0 → scalar plane filled with 0
    let hmc_idx = buffer.schema().index_of("halfmove_clock").unwrap();
    assert!(buffer.as_slice()[hmc_idx * 64..hmc_idx * 64 + 64]
        .iter()
        .all(|&x| x == 0.0),
        "Halfmove clock plane should be all 0s at start");

    // Threefold count = 0 → scalar plane filled with 0
    let tf_idx = buffer.schema().index_of("threefold_count").unwrap();
    assert!(buffer.as_slice()[tf_idx * 64..tf_idx * 64 + 64]
        .iter()
        .all(|&x| x == 0.0),
        "Threefold count plane should be all 0s at start");
}