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

    let moves = get_legal_moves(board, color);

    let mut total_nodes = 0u64;

    for mve in moves {

        board.make_move_in_place(mve);

        total_nodes += perft(board, depth - 1, color.opposite_color()); 
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

pub fn divide(board: &mut Board, depth: u64, color: Color) -> u64{

    let moves = get_legal_moves(board, color);

    let mut total_nodes = 0u64;

    for mve in moves {

        let mv_string = mve.to_string(); 

        board.make_move_in_place(mve);
        
        let count = perft(board, depth - 1, color.opposite_color());
        board.unmake_move();

        println!("{}: {}", mv_string, count); 
        total_nodes += count; 
        
    }

    total_nodes
    
}

pub fn check_h2h3_move(board: &mut Board, color: Color) {
    let moves = get_legal_moves(board, color);

    for mve in moves {
        let mv_string = mve.to_string(); 
        if mv_string == "h2h3" {
            let mut board_copy = board.clone();
            board_copy.make_move_in_place(mve);
            let replies = get_legal_moves(&board_copy, color.opposite_color());
            println!("Replies after a2a4 ({} moves):", replies.len());
            for r in &replies {
                println!("{}", r);
            }
        }

    }
}