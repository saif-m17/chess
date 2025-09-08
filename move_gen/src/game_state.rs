use crate::bitboards::{*};
use crate::board::Board;
use crate::movegen::{get_captures, get_pseudo_legal_moves, has_legal_move, is_in_check, moves_into_check};
use crate::moves::{Color, Color::{*}, Move, Piece, Piece::{*}, Square, CastlingSide};
use crate::utils::MoveList;
use crate::action_space::MoveIntent; 
use std::collections::HashMap; 

const GET_PIECE_BITBOARD: [fn(&Board, Color) -> Bitboard; 6] = [
    Board::get_pawn_bb,
    Board::get_knight_bb,
    Board::get_bishop_bb,
    Board::get_rook_bb,
    Board::get_queen_bb,
    Board::get_king_bb,
]; 

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outcome {
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
    half_move_clock: u32,
    side: Color,
    outcome: Option<Outcome>,
    winner: Option<Color>,
    past_states: HashMap<u64, u8>,
    pseudo_legal_moves: MoveCache,
    legal_moves: MoveCache,
    captures: MoveCache,
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
            captures: MoveCache::new(),
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
            captures: MoveCache::new(),
            current_hash: 0u64,
        }; 

        let hash = game_state.get_zobrist(); 
        game_state.past_states.entry(hash).and_modify(|count| *count += 1).or_insert(1); 
        game_state.current_hash = hash; 

        get_pseudo_legal_moves(&game_state.board, game_state.side, &mut game_state.pseudo_legal_moves.get_cache());
        game_state.pseudo_legal_moves.set_cached(true);
        for mv in game_state.pseudo_legal_moves.get_cache().iter() {
            if !moves_into_check(&game_state.board, game_state.side, mv) {
                game_state.legal_moves.get_cache().push(*mv); 
            }
        }
        game_state.legal_moves.set_cached(true);

        if game_state.legal_moves.get_cache().len() == 0 {
            if is_in_check(&game_state.board, game_state.side) {
                game_state.outcome = Some(Outcome::Checkmate);
                game_state.winner = Some(game_state.side.opposite_color());
            } else {
                game_state.outcome = Some(Outcome::Draw);
            }
        }

        Ok(game_state)
    }   

    pub fn get_outcome(&self) -> Option<Outcome> {
        self.outcome 
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_side(&self) -> Color {
        self.side
    }

    /// Makes move & checks for legality / outcomes
    pub fn make_move(&mut self, mv: Move) -> Result<(), MoveError>{
        if self.get_outcome().is_some() {
            return Err(MoveError::IllegalMove); 
        }
        let old_castling = self.board.enumerate_castling();
        let old_en_passant_square = self.board.get_en_passant_square(); 

        if self.legal_moves.is_cached() {

            if self.legal_moves.get_cache().contains(&mv) {
                self.board.make_move_in_place(mv);
            } else {
                return Err(MoveError::IllegalMove)
            }

        } else {
            if !self.pseudo_legal_moves.is_cached() {
                get_pseudo_legal_moves(&self.board, self.side, self.pseudo_legal_moves.get_cache());
            }

            if self.pseudo_legal_moves.get_cache().contains(&mv) {
                self.board.make_move_in_place(mv);
                if is_in_check(&self.board, self.side) {
                    self.board.unmake_move();
                    return Err(MoveError::IllegalMove)
                }
            } else {
                return Err(MoveError::IllegalMove)
            }

        }

        self.move_number += 1;
        if mv.piece != Pawn && !mv.is_capture() {
            self.half_move_clock += 1; 
        } else {
            self.half_move_clock = 0; 
        } 

        self.legal_moves.get_cache().clear();
        self.legal_moves.set_cached(false);
        self.pseudo_legal_moves.get_cache().clear();

        self.side = self.side.opposite_color(); 

        // Checking for checkmate
        get_pseudo_legal_moves(&self.board, self.side, &mut self.pseudo_legal_moves.get_cache());

        let has_legal_moves = has_legal_move(&mut self.board, self.side, &self.pseudo_legal_moves.get_cache()); 

        if !has_legal_moves {
            if is_in_check(&self.board, self.side) {
                self.outcome = Some(Outcome::Checkmate);
                self.winner = Some(self.side.opposite_color());
                return Ok(());  
            } else {
                self.outcome = Some(Outcome::Draw);
                return Ok(()); 
            }
        }

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

    }

    pub fn unmake_move(&mut self) {
        let last_move = *self.board.get_last_move().unwrap(); 
        let old_castling = self.board.enumerate_castling();
        let old_ep_square = self.board.get_en_passant_square(); 
        self.board.unmake_move();

        let new_castling = self.board.enumerate_castling();
        let new_ep_square = self.board.get_en_passant_square();

        self.update_zobrist(&last_move, old_castling, &old_ep_square, new_castling, &new_ep_square);
        self.past_states.entry(self.current_hash).and_modify(|count| *count -= 1); 

        self.legal_moves.get_cache().clear();
        self.legal_moves.set_cached(false);
        self.pseudo_legal_moves.get_cache().clear();
        self.pseudo_legal_moves.set_cached(false);

        if last_move.piece != Pawn && !last_move.is_capture() {
            self.half_move_clock -= 1; 
        }

        self.side = self.side.opposite_color();

        self.outcome = None;
        self.winner = None;

    }

    pub fn get_gamestate_pseudo_legal_moves(&mut self) -> &MoveList {
        if self.pseudo_legal_moves.is_cached() {
            self.pseudo_legal_moves.get_cache()
        } else {
            self.pseudo_legal_moves.clear_cache();
            get_pseudo_legal_moves(&self.board, self.side, &mut self.pseudo_legal_moves.get_cache());
            self.pseudo_legal_moves.set_cached(true);
            self.pseudo_legal_moves.get_cache()
        }
    }

    pub fn get_gamestate_legal_moves(&mut self) -> &MoveList {
        if self.legal_moves.is_cached() {
            self.legal_moves.get_cache()
        } else if self.pseudo_legal_moves.is_cached() {
            self.legal_moves.clear_cache();
            for mv in self.pseudo_legal_moves.get_cache().iter() {
                self.legal_moves.get_cache().push(*mv); 
            }
            self.legal_moves.set_cached(true);
            self.legal_moves.get_cache() 
        } else {
            get_pseudo_legal_moves(&self.board, self.side, &mut self.pseudo_legal_moves.get_cache());
            self.pseudo_legal_moves.set_cached(true);
            for mv in self.pseudo_legal_moves.get_cache().iter() {
                if !moves_into_check(&self.board, self.side, mv) {
                    self.legal_moves.get_cache().push(*mv); 
                }
            }
            self.legal_moves.set_cached(true);
            self.legal_moves.get_cache() 
        }
    }

    pub fn get_gamestate_captures(&mut self) -> &MoveList {
        if self.captures.is_cached() {
            self.captures.get_cache()
        } else {
            get_captures(&self.board, self.side, &mut self.captures.get_cache());
            self.captures.get_cache()
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
                let file =(en_passant_square.unwrap() as usize) % 8; 
                zobrist_hash ^= ZOBRIST_EN_PASSANT[file]
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

    ///
    pub fn realize_move(&mut self, intent: MoveIntent) -> Result<Move, &'static str> {
        // Normal, Castle { kingside: bool }, En Passant, DoublePawnPush, Promotion { piece: Piece }
        let piece = self.board.get_piece_at(intent.from() as u64).expect("move should be valid."); 
    
        let color = if GET_PIECE_BITBOARD[piece as usize](&self.board, Color::White) & intent.from().to_bitboard() != 0 {
            Color::White
        } else {
            Color::Black
        }; 
    
        let from_row = (intent.from() as u8) / 8;
        let from_file = (intent.from() as u8) % 8;
    
        let to_row = (intent.to() as u8) / 8;
        let to_file = (intent.to() as u8) % 8;
    
        let drow = (to_row as i8)  - (from_row as i8);
        let dfile = (to_file as i8) - (from_file as i8); 
    
        if piece == Piece::King && intent.from() == KING_INITIAL_SQUARE[color as usize] && dfile.abs() == 2 {
            if intent.to() == CASTLING_DESTINATION_SQUARES[color as usize][CastlingSide::Queenside as usize] {
                return Ok(Move::new_castle(color, false))
            } else {
                return Ok(Move::new_castle(color, true))
            }
        }
    
        if piece == Piece::Pawn {
            if drow.abs() == 2 {
                return Ok(Move::new_double_pawn_push(intent.from(), intent.to(), color)); 
            }
            if intent.from().to_bitboard() & PAWN_PROMOTION_RANK[color as usize] != 0 {
                return Ok(Move::new_promotion(intent.from(), intent.to(), color, self.board.get_piece_at(intent.to() as u64), 
                intent.promotion().expect("move should be valid"))); 
            }
            if drow.abs() == 1 && dfile.abs() == 1 && self.board.get_piece_at(intent.to() as u64).is_none() {
                return Ok(Move::new_en_passant(intent.from(), intent.to(), color)); 
            }
        }
    
        Ok(Move::new_normal(
            intent.from(),
            intent.to(), 
            self.board.get_piece_at(intent.from() as u64).expect("move should be valid"),
            color, 
            self.board.get_piece_at(intent.to() as u64))
        )
    
    }

    pub fn display_board(&self) {
        self.board.display()
    }

    pub fn get_winner(&self) -> Option<Color> {
        self.winner
    }

    pub fn get_half_move_clock(&self) -> u32 {
        self.half_move_clock
    }

    pub fn get_three_fold_count(&self) -> u8 {
        *self.past_states.get(&self.current_hash).unwrap_or(&0)
    }

}