use crate::board::{*};
use crate::movegen::{*};
use crate::moves::{*}; 

// Structs to reduce memory calls
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveList {
    pub moves: [Move; 256],
    pub len: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [Move::default(); 256],
            len: 0,
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    #[inline(always)]
    pub fn push(&mut self, mve: Move) {
        self.moves[self.len] = mve;
        self.len += 1;
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves[..self.len].iter()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<Move> {
        if index < self.len {
            Some(self.moves[index])
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn contains(&self, m: &Move) -> bool {
        self.moves[..self.len].contains(m)
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&Move) -> bool,
    {
        let mut new_len = 0;

        for i in 0..self.len {
            if f(&self.moves[i]) {
                self.moves[new_len] = self.moves[i];
                new_len += 1;
            }
        }

        self.len = new_len;
    }
}

pub fn perft_shared_memory(board: &mut Board, depth: u64, color: Color) -> u64 {
    let mut buffers = [MoveList::new(); 64];
    perft_recursive_shared_mem(board, depth, color, &mut buffers, 0)
}

pub fn perft_recursive_shared_mem(board: &mut Board, depth: u64, color: Color, 
    move_buffers: &mut [MoveList], ply: usize) -> u64 {

        let (curr_buffer, rest) = move_buffers.split_at_mut(ply + 1);
        let buffer = &mut curr_buffer[ply];
        buffer.clear(); 

        if depth == 1 {
            get_legal_moves(board, color, buffer);
            return buffer.len() as u64; 
        }

        get_pseudo_legal_moves(board, color, buffer);
    
        let mut nodes = 0;
        
        for &mve in buffer.iter() {
            board.make_move_in_place(mve);
    
            if !is_in_check(board, color) {
                nodes += perft_recursive_shared_mem(board, depth - 1, color.opposite_color(), rest, ply + 1);
    
            }
    
            board.unmake_move();
        }
        nodes

}



// pub fn perft(board: &mut Board, depth: u64, color: Color) -> u64 {
//     let mut move_buffer = Vec::with_capacity(256);
//     perft_recursive(board, depth, color, &mut move_buffer)
// }

// fn perft_recursive(board: &mut Board, depth: u64, color: Color, move_buffer: &mut Vec<Move>) -> u64 {

//     move_buffer.clear();

//     if depth == 1 { 
//         get_legal_moves(board, color, move_buffer);
//         return move_buffer.len() as u64; 
//     }

//     get_pseudo_legal_moves(board, color, move_buffer);
    
//     let mut nodes = 0;
//     let currently_in_check = is_in_check(board, color); 
    
//     for i in 0..move_buffer.len() {
//         let mve = move_buffer[i];
//         if let MoveType::Castle { kingside: _ } = mve.move_type {
//             if currently_in_check {
//                 continue; 
//             }
//         }
//         board.make_move_in_place(mve);

//         if !is_in_check(board, color) {
//             let mut child_buffer = Vec::with_capacity(256);
//             nodes += perft_recursive(board, depth - 1, color.opposite_color(), &mut child_buffer);

//         }

//         board.unmake_move();
//     }
//     nodes
// }

// pub fn divide(board: &mut Board, depth: u64, color: Color) -> u64 {
//     let mut move_buffer = Vec::with_capacity(256);
//     if depth == 1 {
//         get_legal_moves(board, color, &mut move_buffer);
//         return move_buffer.len() as u64;
//     }

//     let mut total_nodes = 0u64;

//     get_pseudo_legal_moves(board, color, &mut move_buffer);
//     let currently_in_check = is_in_check(board, color);

//     for i in 0..move_buffer.len() {
//         let mve = move_buffer[i]; 
//         let mv_string = mve.to_string(); 
//         if let MoveType::Castle { kingside: _ } = mve.move_type {
//             if currently_in_check {
//                 continue; 
//             }
//         }
//         board.make_move_in_place(mve);
//         if !is_in_check(board, color) {
//             let count = perft(board, depth - 1, color.opposite_color());
//             total_nodes += count; 
//             println!("{}: {}", mv_string, count); 
//         }
//         board.unmake_move();
        
//     }

//     total_nodes
// }

pub fn square_to_string(sq: u8) -> String {
    let file = (sq % 8) as u8;
    let rank = (sq / 8) as u8;
    let file_char = (b'a' + file) as char;
    let rank_char = (b'1' + rank) as char;
    format!("{}{}", file_char, rank_char)
}

