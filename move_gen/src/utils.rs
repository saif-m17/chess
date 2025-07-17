use crate::board::{*};
use crate::movegen::{*};
use crate::moves::{*}; 
use crate::bitboards::{*}; 

// Utility functions for testing

fn assert_move_count(board: &Board, expected: usize) {
    todo!()
}

fn assert_contains_move(moves: &[Move], expected_move: Move) {
    todo!()
}

fn assert_board_state_equal(board1: &Board, board2: &Board) {
    todo!()
}

fn generate_random_legal_position() -> Board {
    todo!()
}


pub fn perft(board: &mut Board, depth: u64, color: Color) -> u64{
    if depth == 0 {
        return 1;
    }

    let moves = get_check_aware_pseudo_legal_moves(board, color);

    let mut total_nodes = 0u64;

    for mve in moves {
        let to = mve.to as u64;
        let from = mve.from as u64;

        board.make_move_in_place(mve);

        if is_in_check(board, color) {
            board.unmake_move();
            continue;
        }

        total_nodes += perft(board, depth - 1, color.opposite_color()); 
        board.unmake_move();
    }

    total_nodes
    
}