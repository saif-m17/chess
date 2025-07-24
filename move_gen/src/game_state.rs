use crate::bitboards::{*};
use crate::board::Board;
use crate::movegen::{get_pseudo_legal_moves, has_legal_move, is_in_check};
use crate::moves::{Color, Color::{*}, Move, Piece::{*}, Square};
use crate::utils::MoveList;
use std::collections::HashMap; 

enum Outcome {
    Draw = 0,
    Checkmate = 1,
}

struct MoveCache {
    moves: MoveList,
    cached: bool,
}

impl MoveCache {
    pub fn new() -> Self {
        let moves = MoveList::new();
        let cached = false;
        MoveCache { moves, cached }
    }

    pub fn is_cached(&self) -> bool {
        self.cached
    }

    pub fn get_cache(&mut self) -> &mut MoveList {
        &mut self.moves
    }

    pub fn set_cached(&mut self, is_cached: bool) {
        self.cached = is_cached; 
    }

    pub fn clear_cache(&mut self) {
        self.moves.clear(); 
    }
}

pub struct GameState {
    board: Board,
    move_number: u64,
    half_move_clock: u64,
    side: Color,
    outcome: Option<Outcome>,
    winner: Option<Color>,
    past_states: HashMap<u64, u8>,
    pseudo_legal_moves: MoveCache,
    legal_moves: MoveCache,
    current_hash: u64,

}

impl GameState {
    pub fn new() -> Self {
        let board = Board::new();
        let mut game_state = GameState {
            board,
            outcome: None,
            move_number: 0,
            half_move_clock: 0,
            side: White,
            winner: None,
            past_states: HashMap::new(),
            pseudo_legal_moves: MoveCache::new(),
            legal_moves: MoveCache::new(),
            current_hash: 0u64,
        }; 
        let hash = game_state.get_zobrist(); 
        game_state.past_states.entry(hash).and_modify(|count| *count += 1).or_insert(1); 
        game_state.current_hash = hash; 
        game_state
    }

    pub fn from_fen(fen_string: &str) -> Result<Self, FenError> {
        let parts: Vec<&str> = fen_string.split_whitespace().collect();
        if parts.len() != 6 { return Err(FenError::InvalidFormat("Length incorrect.".to_string())) }

        let board = Board::from_fen(fen_string)?;
        let move_number = parts[5].parse().map_err(|_| FenError::InvalidFormat("Move number incorrect.".to_string()))?;
        let half_move_clock = parts[4].parse().map_err(|_| FenError::InvalidFormat("half number clock incorrect.".to_string()))?; 
        let side =  match parts[1] {
            "w" => White,
            "b" => Black,
            _ => return Err(FenError::InvalidFormat("Side incorrect".to_string())),
        }; 

        let mut game_state = GameState {
            board,
            move_number, 
            half_move_clock,
            side,
            outcome: None,
            winner: None,
            past_states: HashMap::new(),
            pseudo_legal_moves: MoveCache::new(),
            legal_moves: MoveCache::new(),
            current_hash: 0u64,
        }; 

        let hash = game_state.get_zobrist(); 
        game_state.past_states.entry(hash).and_modify(|count| *count += 1).or_insert(1); 
        game_state.current_hash = hash; 

        Ok(game_state)
    }   

    /// Makes move & checks for legality / outcomes
    pub fn make_move(&mut self, mv: Move) -> Result<(), MoveError>{
        if !self.pseudo_legal_moves.is_cached() {
            get_pseudo_legal_moves(&self.board, self.side, self.pseudo_legal_moves.get_cache());
        }

        let currently_in_check = is_in_check(&self.board, self.side); 
        let has_legal_moves = has_legal_move(&mut self.board, self.side, &self.pseudo_legal_moves.get_cache()); 

        // Checking for checkmate or stalemate
        if !has_legal_moves {
            if currently_in_check {
                self.outcome = Some(Outcome::Checkmate);
                self.winner = Some(self.side.opposite_color());
                return Ok(());  
            } else {
                self.outcome = Some(Outcome::Draw);
                return Ok(()); 
            }
        }

        let old_castling = self.board.enumerate_castling();
        let old_en_passant_square = self.board.get_en_passant_square(); 

        if self.pseudo_legal_moves.get_cache().contains(&mv) {
            self.board.make_move_in_place(mv);
            if is_in_check(&self.board, self.side) {
                self.board.unmake_move();
                return Err(MoveError::IllegalMove)
            }

            self.move_number += 1;
            if mv.piece != Pawn && !mv.is_capture() {
                self.half_move_clock += 1; 
            } else {
                self.half_move_clock = 0; 
            } 

            self.pseudo_legal_moves.get_cache().clear();
            self.pseudo_legal_moves.set_cached(false);
            self.side = self.side.opposite_color(); 

            // Checking for threefold repition or 50 move rule
            let new_castling = self.board.enumerate_castling(); 
            let new_ep_square = &self.board.get_en_passant_square(); 
            self.update_zobrist(&mv, old_castling, &old_en_passant_square, new_castling, new_ep_square); 
            self.past_states.entry(self.current_hash).and_modify(|count| *count += 1).or_insert(1); 

            if self.past_states[&self.current_hash] >= 3 || self.half_move_clock >= 50 {
                self.outcome = Some(Outcome::Draw); 
                return Ok(()); 
            }

            Ok(())
        } else {
            Err(MoveError::IllegalMove)
        }

    }

    pub fn get_pseudo_legal_moves(&mut self) -> &MoveList {
        if self.pseudo_legal_moves.is_cached() {
            self.pseudo_legal_moves.get_cache()
        } else {
            self.pseudo_legal_moves.clear_cache();
            get_pseudo_legal_moves(&self.board, self.side, &mut self.pseudo_legal_moves.get_cache());
            self.pseudo_legal_moves.set_cached(true);
            self.pseudo_legal_moves.get_cache()
        }
    }

    /// Function to initialize zobrist hash from either FEN or regular starting position.
    fn get_zobrist(&self) -> u64 {
        let mut zobrist_hash = 0u64; 
        for square in 0..64 {
            let piece = self.board.get_piece_lookup()[square]; 
            if piece.is_some() {
                if self.board.get_piece_lists()[White as usize][piece.expect("Piece is some.") as usize].get_bit(square as u64) {
                    zobrist_hash ^= ZOBRIST_TABLE[White as usize][piece.expect("Piece is explicitly some") as usize][square as usize]; 
                } else {
                    zobrist_hash ^= ZOBRIST_TABLE[Black as usize][piece.expect("Piece is explicitly some") as usize][square as usize];
                }
            }
        }

        zobrist_hash ^= ZOBRIST_CASTLING[self.board.enumerate_castling() as usize]; 
        zobrist_hash ^= ZOBRIST_IS_BLACK[self.side as usize];

        let en_passant_square = self.board.get_en_passant_square(); 
        if en_passant_square.is_some() {
            if self.board.get_pawn_bb(self.side.opposite_color()) & en_passant_square.unwrap().to_bitboard() != 0 {
                zobrist_hash ^= ZOBRIST_EN_PASSANT[en_passant_square.unwrap() as usize % 8]
            }
        }

        zobrist_hash
    }

    /// Update hash after a given move is made.
    fn update_zobrist(&mut self, mv: &Move, old_castling: u8, old_ep_square: &Option<Square>, new_castling: u8, 
        new_ep_square: &Option<Square>) {

        self.current_hash ^= ZOBRIST_TABLE[mv.color as usize][mv.piece as usize][mv.from as usize];
        self.current_hash ^= ZOBRIST_TABLE[mv.color as usize][mv.piece as usize][mv.to as usize];

        if let Some(captured) = mv.captured {
            self.current_hash ^= ZOBRIST_TABLE[mv.color.opposite_color() as usize][captured as usize][mv.to as usize]; 
        }

        self.current_hash ^= ZOBRIST_CASTLING[old_castling as usize];
        self.current_hash ^= ZOBRIST_CASTLING[new_castling as usize];

        if let Some(ep_square) = old_ep_square {
            self.current_hash ^= ZOBRIST_EN_PASSANT[*ep_square as usize]; 
        }

        if let Some(ep_square) = new_ep_square {
            self.current_hash ^= ZOBRIST_EN_PASSANT[*ep_square as usize]; 
        }

        self.current_hash ^= ZOBRIST_IS_BLACK[Black as usize]; 
    }

}