use crate::board::{*};
use crate::movegen::{*};
use crate::moves::{*}; 

// Utility functions for testing

pub fn perft(board: &mut Board, depth: u64, color: Color) -> u64 {
    let mut move_buffer = Vec::with_capacity(256);
    perft_recursive(board, depth, color, &mut move_buffer)
}

fn perft_recursive(board: &mut Board, depth: u64, color: Color, move_buffer: &mut Vec<Move>) -> u64 {

    move_buffer.clear();

    if depth == 1 { 
        get_legal_moves(board, color, move_buffer);
        return move_buffer.len() as u64; 
    }

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
    if depth == 1 {
        get_legal_moves(board, color, &mut move_buffer);
        return move_buffer.len() as u64;
    }

    let mut total_nodes = 0u64;

    get_pseudo_legal_moves(board, color, &mut move_buffer);
    let currently_in_check = is_in_check(board, color);

    for i in 0..move_buffer.len() {
        let mve = move_buffer[i]; 
        let mv_string = mve.to_string(); 
        if let MoveType::Castle { kingside: _ } = mve.move_type {
            if currently_in_check {
                continue; 
            }
        }
        board.make_move_in_place(mve);
        if !is_in_check(board, color) {
            let count = perft(board, depth - 1, color.opposite_color());
            total_nodes += count; 
            println!("{}: {}", mv_string, count); 
        }
        board.unmake_move();
        
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

