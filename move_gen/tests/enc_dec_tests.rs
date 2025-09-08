use chess_core::game_state::{GameState, Outcome}; 
use chess_core::tensor::{TensorBuffer, FeatureSchema};
use chess_core::action_space::{encode_action, decode_action, realize_move};

#[test]
fn encode_action_test() {
    let mut game = GameState::new(); 
    let board = game.get_board().clone(); 
    let moves = game.get_gamestate_legal_moves(); 

    for mv in moves.iter() {
        let encoding = encode_action(mv);
        let intent = decode_action(encoding).expect("move should be legal.");
        let decoded_move = realize_move(&board, intent).expect("move decoding should work."); 
        assert_eq!(*mv, decoded_move, "Move: {:?} did not roundtrip correctly", mv); 
    }
}

#[test]
fn encode_action_test_kiwipete() {
    let mut game = GameState::from_fen("r3kq1r/p1pp1pb1/bn2pQp1/3PN3/1p2P3/2N4p/PPPBBPPP/R3K2R w KQkq - 1 2").expect("valid fen."); 
    let board = game.get_board().clone(); 
    let moves = game.get_gamestate_legal_moves(); 

    for mv in moves.iter() {
        let encoding = encode_action(mv);
        let intent = decode_action(encoding).expect("move should be legal.");
        let decoded_move = realize_move(&board, intent).expect("move decoding should work."); 
        assert_eq!(*mv, decoded_move, "Move: {:?} did not roundtrip correctly", mv); 
    }
}

#[test]
fn encode_action_test_position3cpw() {
    let mut game = GameState::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").expect("valid fen."); 
    let board = game.get_board().clone(); 
    let moves = game.get_gamestate_legal_moves(); 

    for mv in moves.iter() {
        let encoding = encode_action(mv);
        let intent = decode_action(encoding).expect("move should be legal.");
        let decoded_move = realize_move(&board, intent).expect("move decoding should work."); 
        assert_eq!(*mv, decoded_move, "Move: {:?} did not roundtrip correctly", mv); 
    }
}

#[test]
fn encode_action_test_promocheck() {
    let mut game = GameState::from_fen("8/8/3p4/KP5r/1R3p1k/8/2p1P1P1/8 b - - 1 5").expect("valid fen."); 
    let board = game.get_board().clone(); 
    let moves = game.get_gamestate_legal_moves(); 

    for mv in moves.iter() {
        let encoding = encode_action(mv);
        let intent = decode_action(encoding).expect("move should be legal.");
        let decoded_move = realize_move(&board, intent).expect("move decoding should work."); 
        assert_eq!(*mv, decoded_move, "Move: {:?} did not roundtrip correctly", mv); 
    }
}

