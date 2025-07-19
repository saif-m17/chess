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

pub fn perft(board: &mut Board, depth: u64, color: Color) -> u64 {
    let mut move_buffer = Vec::with_capacity(256);
    perft_recursive(board, depth, color, &mut move_buffer)
}

fn perft_recursive(board: &mut Board, depth: u64, color: Color, move_buffer: &mut Vec<Move>) -> u64 {
    if depth == 0 { return 1; }
    
    move_buffer.clear();
    get_pseudo_legal_moves(board, color, move_buffer);
    
    let mut nodes = 0;
    let currently_in_check = is_in_check(board, color); 
    
    for i in 0..move_buffer.len() {
        let mve = move_buffer[i];
        if let MoveType::Castle { kingside: _ } = mve.move_type {
            if currently_in_check {
                continue; 
            }
        }
        board.make_move_in_place(mve);

        if !is_in_check(board, color) {
            let mut child_buffer = Vec::with_capacity(256);
            nodes += perft_recursive(board, depth - 1, color.opposite_color(), &mut child_buffer);

        }

        board.unmake_move();
    }
    nodes
}

pub fn divide(board: &mut Board, depth: u64, color: Color) -> u64 {
    let mut move_buffer = Vec::with_capacity(256);
    get_legal_moves(board, color, &mut move_buffer);

    let mut total_nodes = 0u64;
    
    let moves_to_process: Vec<Move> = move_buffer.iter().copied().collect();

    for mve in moves_to_process {
        let mv_string = mve.to_string(); 

        board.make_move_in_place(mve);
        
        let count = perft(board, depth - 1, color.opposite_color());
        board.unmake_move();

        println!("{}: {}", mv_string, count); 
        total_nodes += count; 
    }

    total_nodes
}

pub fn square_to_string(sq: u8) -> String {
    let file = (sq % 8) as u8;
    let rank = (sq / 8) as u8;
    let file_char = (b'a' + file) as char;
    let rank_char = (b'1' + rank) as char;
    format!("{}{}", file_char, rank_char)
}

