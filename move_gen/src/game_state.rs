use crate::bitboards::{FenError, MoveError};
use crate::board::Board;
use crate::movegen::{get_pseudo_legal_moves, is_in_check};
use crate::moves::{Color, Color::{*}, Move, Piece::{*}};
use crate::utils::MoveList; 

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

struct GameState {
    board: Board,
    move_number: u64,
    half_move_clock: u64,
    side: Color,
    outcome: Option<Outcome>,
    winner: Option<Color>,
    past_states: Vec<Board>, // change this to some hashed version
    pseudo_legal_moves: MoveCache,
    legal_moves: MoveCache,
}

impl GameState {
    pub fn new() -> Self {
        let board = Board::new();
        GameState {
            board,
            outcome: None,
            move_number: 0,
            half_move_clock: 0,
            side: White,
            winner: None,
            past_states: Vec::new(),
            pseudo_legal_moves: MoveCache::new(),
            legal_moves: MoveCache::new(),
        }
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
        Ok(GameState {
            board,
            move_number, 
            half_move_clock,
            side,
            outcome: None,
            winner: None,
            past_states: Vec::new(),
            pseudo_legal_moves: MoveCache::new(),
            legal_moves: MoveCache::new(),
        })
    }   

    /// Makes move & checks for legality
    pub fn make_move(&mut self, mv: Move) -> Result<(), MoveError>{
        if !self.pseudo_legal_moves.is_cached() {
            get_pseudo_legal_moves(&self.board, self.side, self.pseudo_legal_moves.get_cache());
        }

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

}