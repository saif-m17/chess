use crate::bitboards::{FenError, MoveError};
use crate::board::Board;
use crate::movegen::{get_pseudo_legal_moves, is_in_check};
use crate::moves::{Color, Move};
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
            winner: None,
            past_states: Vec::new(),
            pseudo_legal_moves: MoveCache::new(),
            legal_moves: MoveCache::new(),
        }
    }

    pub fn from_fen(fen_string: &str) -> Result<Self, FenError> {
        let board = Board::from_fen(fen_string)?;
        Ok(GameState {
            board,
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
            get_pseudo_legal_moves(&self.board, self.board.get_color(), self.pseudo_legal_moves.get_cache());
        }

        if self.pseudo_legal_moves.get_cache().contains(&mv) {
            self.board.make_move_in_place(mv);
            if is_in_check(&self.board, self.board.get_color()) {
                self.board.unmake_move();
                return Err(MoveError::IllegalMove)
            }
            self.pseudo_legal_moves.get_cache().clear();
            self.pseudo_legal_moves.set_cached(false);
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
            get_pseudo_legal_moves(&self.board, self.board.get_color(), &mut self.pseudo_legal_moves.get_cache());
            self.pseudo_legal_moves.set_cached(true);
            self.pseudo_legal_moves.get_cache()
        }
    }

}