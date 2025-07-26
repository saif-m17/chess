use chess_core::game_state::{GameState, Outcome}; 
use chess_core::moves::{Move, Square, Piece::{*}, Color::{*}}; 

#[test]
fn test_new_game_state() {
    let game_state = GameState::new();
    game_state.display_board();
}

#[test]
fn test_checkmate_detection_from_starting_position() {
    let mut game_state = GameState::from_fen("rnbqkbnr/2pp1Qpp/pp6/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4").unwrap();
    game_state.display_board();
    assert!(game_state.get_gamestate_legal_moves().len() == 0); 
    assert!(game_state.get_outcome().expect("Position is checkmate.") == Outcome::Checkmate)
}

#[test]
fn test_checkmate_detection_making_move() {
    let mut game_state = GameState::from_fen("rnbqkbnr/2pp1ppp/pp6/4p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR w KQkq - 0 4").unwrap();
    let qf7 = Move::new_normal(Square::F3, Square::F7, Queen, White, Some(Pawn)); 
    game_state.make_move(qf7).expect("Legal Move."); 
    assert!(game_state.get_outcome().expect("Position is checkmate.") == Outcome::Checkmate); 
    assert!(game_state.get_winner().expect("Winner is white.") == White); 
}