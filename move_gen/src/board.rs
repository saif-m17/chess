use std::fmt::Error;

use crate::bitboards::{*}; 
use crate::moves::{Color::{self, *}, Move, MoveType, Piece::{self, *}, Square::{self, *}};

#[derive(Clone)]
pub struct Board {
    pub pieces: [[Bitboard; 6]; 2],
    pub piece_lookup: [Option<Piece>; 64],
    pub past_moves: Vec<Move>,
    pub en_passant_square: Option<Square>,
    pub prev_en_passant_squares: Vec<Option<Square>>,
    pub move_changed_castling_rights: [[i32; 2]; 2],
    pub move_number: u64,
    pub half_move_clock: u64,
    pub side: Color,
}

impl Board {
    /// Creates a new board in the standard starting position.
    pub fn new() -> Board {
        let mut pieces = [[0u64;  6]; 2]; 
            
        // White Pieces 
        pieces[White as usize][Pawn as usize] = RANK_2;
        pieces[White as usize][Knight as usize] = B1.to_bitboard() | G1.to_bitboard();
        pieces[White as usize][Bishop as usize] = C1.to_bitboard() | F1.to_bitboard(); 
        pieces[White as usize][Rook as usize] = A1.to_bitboard() | H1.to_bitboard();
        pieces[White as usize][King as usize] = E1.to_bitboard();
        pieces[White as usize][Queen as usize] = D1.to_bitboard();

        // Black Pieces
        pieces[Black as usize][Pawn as usize] = RANK_7;
        pieces[Black as usize][Knight as usize] = B8.to_bitboard() | G8.to_bitboard();
        pieces[Black as usize][Bishop as usize] = C8.to_bitboard() | F8.to_bitboard(); 
        pieces[Black as usize][Rook as usize] = A8.to_bitboard() | H8.to_bitboard();
        pieces[Black as usize][King as usize] = E8.to_bitboard();
        pieces[Black as usize][Queen as usize] = D8.to_bitboard();

        let piece_lookup: [Option<Piece>; 64] = [
            Some(Rook), Some(Knight), Some(Bishop), Some(Queen), Some(King), Some(Bishop), Some(Knight), Some(Rook),  
            Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn),          
            None, None, None, None, None, None, None, None,   
            None, None, None, None, None, None, None, None,  
            None, None, None, None, None, None, None, None,   
            None, None, None, None, None, None, None, None,   
            Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn), Some(Pawn),         
            Some(Rook), Some(Knight), Some(Bishop), Some(Queen), Some(King), Some(Bishop), Some(Knight), Some(Rook),  
        ];

        let past_moves: Vec<Move> = Vec::new(); 

        let prev_en_passant_squares: Vec<Option<Square>> = Vec::new(); 

        let move_changed_castling_rights = [[-1, -1], [-1, -1]];

        let move_number = 0u64;

        let half_move_clock = 0u64;   

        Board { pieces, piece_lookup, past_moves, en_passant_square:None, 
            prev_en_passant_squares, move_changed_castling_rights, move_number, half_move_clock, side:White }
    }

    /// Turns a FEN string into a new board object
    pub fn from_fen(fen_string: &str) -> Result<Board, FenError> {
        let parts: Vec<&str> = fen_string.split_whitespace().collect();
        if parts.len() != 6 { return Err(FenError::InvalidFormat) }

        let mut board = Board {
            pieces: [[0u64; 6]; 2],
            piece_lookup: [None; 64],
            past_moves: Vec::new(),
            en_passant_square: None,
            prev_en_passant_squares: Vec::new(),
            move_changed_castling_rights: [[1; 2]; 2], // some positive number to default to false
            move_number: parts[5].parse().map_err(|_| FenError::InvalidFormat)?,
            half_move_clock: parts[4].parse().map_err(|_| FenError::InvalidFormat)?,
            side: match parts[1].parse().map_err(|_| FenError::InvalidFormat)? {
                0 => White,
                1 => Black,
                _ => return Err(FenError::InvalidFormat),
            },
        }; 

        let piece_rep = parts[0];

        let mut file = 0u64;
        let mut rank = 7u64; 
        for ch in piece_rep.chars() {
            match ch {
                '/' => {
                    rank -= 1;
                    file = 0u64; 
                }
                '1'..='8' => file += ch.to_digit(10).unwrap() as u64,
                _ => {
                    let (color, piece_type) = match ch {
                        'P' => (White, Pawn), 'N' => (White, Knight), 'B' => (White, Bishop), 
                        'R' => (White, Rook), 'Q' => (White, Queen), 'K' => (White, King),
                        'p' => (Black, Pawn), 'n' => (Black, Knight), 'b' => (Black, Bishop), 
                        'r' => (Black, Rook), 'q' => (Black, Queen), 'k' => (Black, King),
                        _ => return Err(FenError::InvalidPiece(ch)),
                    };
                    let square_index: u64 = rank * 8 + file; 
                    board.pieces[color as usize][piece_type as usize] |= 1u64 << square_index; 
                    board.piece_lookup[square_index as usize] = Some(piece_type); 
                    file += 1; 
                }
            } 
        }

        let castling_rights = parts[2];
        for ch in castling_rights.chars() {
            match ch {
                'K' => board.move_changed_castling_rights[0][1] = -1,
                'Q' => board.move_changed_castling_rights[0][0] = -1,
                'k' => board.move_changed_castling_rights[1][1] = -1,
                'q' => board.move_changed_castling_rights[1][0] = -1,
                '-' => break,
                _ => return Err(FenError::InvalidCastling),
            }
        }

        let en_passant = parts[3];
        for ch in en_passant.chars() {
            if ch != '-' {
                let bytes = parts[3].as_bytes();
                let square = (bytes[1] - b'1') * 8 + (bytes[0] - b'a');
                board.en_passant_square = Some(Square::try_from(square as u64).unwrap());
            }
        } 

        Ok(board)
        
    }

    /// Gets pieces for color 
    pub fn get_pieces(&self, color: Color) -> u64 {
        self.pieces[color as usize].iter().copied().reduce(|a, b| a | b).unwrap_or(0)
    }

    /// Returns position of all pieces
    pub fn get_all_pieces(&self) -> u64 {
        self.get_pieces(Black) | self.get_pieces(White)
    }

    /// Returns piece at a given index if it exists, else None
    pub fn get_piece_at(&self, index: u64) -> Option<Piece> {
        self.piece_lookup[index as usize]
    }

    /// Returns bitboard of all sliding pieces for color.
    pub fn get_sliding_pieces(&self, color: Color) -> Bitboard {
        self.pieces[color as usize][Rook as usize] |
        self.pieces[color as usize][Bishop as usize] |
        self.pieces[color as usize][Queen as usize]
    }

    /// Makes given move by updating current board
    pub fn make_move_in_place(&mut self, mve: Move) {
        self.move_number += 1;
        self.prev_en_passant_squares.push(self.en_passant_square);
        match mve.move_type {
            MoveType::Normal => self.make_normal_move(&mve),
            MoveType::Castle { kingside } => self.make_castle_move(&mve, kingside),
            MoveType::EnPassant => self.make_en_passant_move(&mve),
            MoveType::DoublePawnPush => self.make_double_push_move(&mve),
            MoveType::Promotion { piece } => self.make_promotion_move(&mve, piece),
        }
        if mve.piece != Pawn && !mve.is_capture() {
            self.half_move_clock += 1; 
        } else {
            self.half_move_clock = 0; 
        } 
        self.past_moves.push(mve);
        self.side = self.side.opposite_color();  
    }

    pub fn get_en_passant(&self) -> Option<Bitboard> {
        self.en_passant_square.map(Bitboard::from_square)
    }

    fn make_normal_move(&mut self, mve: &Move) {
        self.pieces[mve.color as usize][mve.piece as usize].clear_bit(mve.from as u64); 
        self.pieces[mve.color as usize][mve.piece as usize].set_bit(mve.to as u64); 

        if let Some(captured_piece) = mve.captured {
            self.pieces[mve.color.opposite_color() as usize][captured_piece as usize].clear_bit(mve.to as u64); 
        }

        if mve.piece == Rook && mve.from == ROOK_CASTLING_INITIAL_SQUARE[mve.color as usize][0] && self.can_castle_queenside(mve.color) {
            self.move_changed_castling_rights[mve.color as usize][0] = self.move_number as i32; 
        } else if mve.piece == Rook && mve.from == ROOK_CASTLING_INITIAL_SQUARE[mve.color as usize][1] && self.can_castle_kingside(mve.color) {
            self.move_changed_castling_rights[mve.color as usize][1] = self.move_number as i32;
        } else if mve.piece == King && self.move_changed_castling_rights[mve.color as usize].iter().any(|&b| b < 0) {
            self.move_changed_castling_rights[mve.color as usize].iter_mut().for_each(|x| {
                if *x < 0 { *x = self.move_number as i32; }
            })
        }

        if self.en_passant_square.is_some() {
            self.en_passant_square = None; 
        }

        self.piece_lookup[mve.from as usize] = None;
        self.piece_lookup[mve.to as usize] = Some(mve.piece); 

    }

    fn make_castle_move(&mut self, mve: &Move, kingside: bool) {
        self.pieces[mve.color as usize][King as usize].clear_bit(mve.from as u64);
        self.pieces[mve.color as usize][King as usize].set_bit(mve.to as u64);

        let rook_to_index = ROOK_CASTLING_DIRECTION[kingside as usize](Bitboard::from_square(mve.to)).trailing_zeros() as u64; 
        let rook_from_index = ROOK_CASTLING_INITIAL_SQUARE[mve.color as usize][kingside as usize] as u64;

        self.pieces[mve.color as usize][Rook as usize].clear_bit(rook_from_index);
        self.pieces[mve.color as usize][Rook as usize].set_bit(rook_to_index); 

        self.piece_lookup[mve.from as usize] = None;
        self.piece_lookup[mve.to as usize] = Some(King);

        self.piece_lookup[rook_from_index as usize] = None;
        self.piece_lookup[rook_to_index as usize] = Some(Rook);

        self.move_changed_castling_rights.iter_mut().flatten().for_each(|x| {
            if *x < 0 { *x = self.move_number as i32; }
        });  

        if self.en_passant_square.is_some() {
            self.en_passant_square = None;
        }

    }

    fn make_double_push_move(&mut self, mve: &Move) {
        self.pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        self.pieces[mve.color as usize][Pawn as usize].set_bit(mve.to as u64);

        self.piece_lookup[mve.from as usize] = None;
        self.piece_lookup[mve.to as usize] = Some(Pawn);

        let to_index = mve.to as u8;
        let offset = OFFSET_SINGLE_PUSH[mve.color as usize] as i16;
        let en_passant_index =  (to_index as i16 - offset) as u64;

        self.en_passant_square = Some(Square::try_from(en_passant_index).unwrap()); 

    }

    fn make_en_passant_move(&mut self, mve: &Move) {
        self.pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        self.pieces[mve.color as usize][Pawn as usize].set_bit(mve.to as u64); 

        self.piece_lookup[mve.from as usize] = None;
        self.piece_lookup[mve.to as usize] = Some(Pawn); 

        let to_index = mve.to as u8;
        let offset = OFFSET_SINGLE_PUSH[mve.color as usize] as i16;
        let captured_piece_index = (to_index as i16 - offset) as u64;

        self.pieces[mve.color.opposite_color() as usize][Pawn as usize].clear_bit(captured_piece_index);
        self.piece_lookup[captured_piece_index as usize] = None;
        self.en_passant_square = None;
    }

    fn make_promotion_move(&mut self, mve: &Move, promotion: Piece) {
        self.pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        self.pieces[mve.color as usize][promotion as usize].set_bit(mve.to as u64); 

        if let Some(captured_piece) = mve.captured {
            self.pieces[mve.color.opposite_color() as usize][captured_piece as usize].clear_bit(mve.to as u64); 
        }

        self.piece_lookup[mve.from as usize] = None;
        self.piece_lookup[mve.to as usize] = Some(Pawn); 

        if self.en_passant_square.is_some() {
            self.en_passant_square = None;
        }

    }

    pub fn make_shallow_move(&self, mve: &Move) -> [[Bitboard; 6]; 2] {
        match mve.move_type {
            MoveType::Normal => self.make_shallow_normal_move(mve),
            MoveType::Castle { kingside } => self.make_shallow_castle_move(mve, kingside),
            MoveType::EnPassant => self.make_shallow_en_passant_move(mve),
            MoveType::DoublePawnPush => self.make_shallow_double_push_move(mve),
            MoveType::Promotion { piece } => self.make_shallow_promotion_move(mve, piece),
        }
    }

    fn make_shallow_normal_move(&self, mve: &Move) -> [[Bitboard; 6]; 2] {
        let mut copied_pieces = self.pieces.clone(); 
        copied_pieces[mve.color as usize][mve.piece as usize].clear_bit(mve.from as u64); 
        copied_pieces[mve.color as usize][mve.piece as usize].set_bit(mve.to as u64); 
        if let Some(captured_piece) = mve.captured {
            copied_pieces[mve.color.opposite_color() as usize][captured_piece as usize].clear_bit(mve.to as u64); 
        }
        copied_pieces
    }

    fn make_shallow_castle_move(&self, mve: &Move, kingside: bool) -> [[Bitboard; 6]; 2] {
        let mut copied_pieces = self.pieces.clone(); 
        copied_pieces[mve.color as usize][King as usize].clear_bit(mve.from as u64);
        copied_pieces[mve.color as usize][King as usize].set_bit(mve.to as u64);

        let rook_to_index = ROOK_CASTLING_DIRECTION[kingside as usize](Bitboard::from_square(mve.to)).trailing_zeros() as u64; 
        let rook_from_index = ROOK_CASTLING_INITIAL_SQUARE[mve.color as usize][kingside as usize] as u64;

        copied_pieces[mve.color as usize][Rook as usize].clear_bit(rook_from_index);
        copied_pieces[mve.color as usize][Rook as usize].set_bit(rook_to_index); 
        copied_pieces
    }

    fn make_shallow_double_push_move(&self, mve: &Move) -> [[Bitboard; 6]; 2] {
        let mut copied_pieces = self.pieces.clone(); 
        copied_pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        copied_pieces[mve.color as usize][Pawn as usize].set_bit(mve.to as u64); 
        copied_pieces
        
    }

    fn make_shallow_en_passant_move(&self, mve: &Move) -> [[Bitboard; 6]; 2] {
        let mut copied_pieces = self.pieces.clone(); 
        copied_pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        copied_pieces[mve.color as usize][Pawn as usize].set_bit(mve.to as u64); 

        let to_index = mve.to as u8;
        let offset = OFFSET_SINGLE_PUSH[mve.color as usize] as i16;
        let captured_piece_index = (to_index as i16 - offset) as u64;

        copied_pieces[mve.color.opposite_color() as usize][Pawn as usize].clear_bit(captured_piece_index);
        copied_pieces

    }

    fn make_shallow_promotion_move(&self, mve: &Move, promotion: Piece) -> [[Bitboard; 6]; 2] {
        let mut copied_pieces = self.pieces.clone(); 
        copied_pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        copied_pieces[mve.color as usize][promotion as usize].set_bit(mve.to as u64); 

        if let Some(captured_piece) = mve.captured {
            copied_pieces[mve.color.opposite_color() as usize][captured_piece as usize].clear_bit(mve.to as u64); 
        }
        copied_pieces
    }

    pub fn unmake_move(&mut self) {
        if let Some(last_move) = self.past_moves.pop() {
            match last_move.move_type {
                MoveType::Normal => self.unmake_normal_move(last_move),
                MoveType::Castle { kingside } => self.unmake_castle_move(last_move, kingside),
                MoveType::DoublePawnPush => self.unmake_double_push_move(last_move),
                MoveType::EnPassant => self.unmake_en_passant_move(last_move),
                MoveType::Promotion { piece } => self.unmake_promotion_move(last_move, piece),
            }
            self.move_number -= 1;
            let prev_ep = self.prev_en_passant_squares.pop().flatten();
            self.en_passant_square = prev_ep;
            self.side = self.side.opposite_color();
        }
    }

    fn unmake_normal_move(&mut self, mve: Move) {

        self.pieces[mve.color as usize][mve.piece as usize].clear_bit(mve.to as u64); 
        self.pieces[mve.color as usize][mve.piece as usize].set_bit(mve.from as u64); 

        if let Some(captured_piece) = mve.captured {
            self.pieces[mve.color.opposite_color() as usize][captured_piece as usize].set_bit(mve.to as u64); 
        }

        if mve.piece == Rook && mve.from == ROOK_CASTLING_INITIAL_SQUARE[mve.color as usize][0] && 
                            self.move_changed_castling_rights[mve.color as usize][0] == self.move_number as i32 {

            self.move_changed_castling_rights[mve.color as usize][0] = -1; 

        } else if mve.piece == Rook && mve.from == ROOK_CASTLING_INITIAL_SQUARE[mve.color as usize][1] &&
                            self.move_changed_castling_rights[mve.color as usize][1] == self.move_number as i32 {

            self.move_changed_castling_rights[mve.color as usize][1] = -1; 

        } else if mve.piece == King && self.move_changed_castling_rights[mve.color as usize].iter().any(|&b| b == self.move_number as i32) {
            self.move_changed_castling_rights[mve.color as usize].iter_mut().for_each(|x| {
                if *x == self.move_number as i32 { 
                    *x = -1; 
                }
            });   
        }

        self.piece_lookup[mve.from as usize] = Some(mve.piece);
        self.piece_lookup[mve.to as usize] = mve.captured; 

    }

    fn unmake_castle_move(&mut self, mve: Move, kingside: bool) {
        self.pieces[mve.color as usize][King as usize].clear_bit(mve.to as u64);
        self.pieces[mve.color as usize][King as usize].set_bit(mve.from as u64);

        let rook_from_index = ROOK_CASTLING_DIRECTION[kingside as usize](Bitboard::from_square(mve.to)).trailing_zeros() as u64; 
        let rook_to_index = ROOK_CASTLING_INITIAL_SQUARE[mve.color as usize][kingside as usize] as u64;

        self.pieces[mve.color as usize][Rook as usize].clear_bit(rook_from_index);
        self.pieces[mve.color as usize][Rook as usize].set_bit(rook_to_index); 

        self.piece_lookup[mve.to as usize] = None;
        self.piece_lookup[mve.from as usize] = Some(King);

        self.piece_lookup[rook_from_index as usize] = None;
        self.piece_lookup[rook_to_index as usize] = Some(Rook);

        self.move_changed_castling_rights.iter_mut().flatten().for_each(|x| {
            if *x == self.move_number as i32 { *x = -1; }
        });  

    }

    fn unmake_double_push_move(&mut self, mve: Move) {
        self.pieces[mve.color as usize][Pawn as usize].clear_bit(mve.to as u64);
        self.pieces[mve.color as usize][Pawn as usize].set_bit(mve.from as u64);

        self.piece_lookup[mve.to as usize] = None;
        self.piece_lookup[mve.from as usize] = Some(Pawn);

    }

    fn unmake_en_passant_move(&mut self, mve: Move) {

        self.pieces[mve.color as usize][Pawn as usize].clear_bit(mve.to as u64);
        self.pieces[mve.color as usize][Pawn as usize].set_bit(mve.from as u64); 

        self.piece_lookup[mve.to as usize] = None;
        self.piece_lookup[mve.from as usize] = Some(Pawn); 

        let to_index = mve.to as u8;
        let offset = OFFSET_SINGLE_PUSH[mve.color as usize] as i16;
        let captured_piece_index = (to_index as i16 - offset) as u64;

        self.pieces[mve.color.opposite_color() as usize][Pawn as usize].set_bit(captured_piece_index);
        self.piece_lookup[captured_piece_index as usize] = Some(Pawn);

    }

    fn unmake_promotion_move(&mut self, mve: Move, promotion: Piece) {

        self.pieces[mve.color as usize][promotion as usize].clear_bit(mve.to as u64);
        self.pieces[mve.color as usize][Pawn as usize].set_bit(mve.from as u64); 

        if let Some(captured_piece) = mve.captured {
            self.pieces[mve.color.opposite_color() as usize][captured_piece as usize].set_bit(mve.to as u64); 
        }

        self.piece_lookup[mve.from as usize] = Some(Pawn);
        self.piece_lookup[mve.to as usize] = mve.captured; 

    }

    pub fn can_castle_queenside(&self, color: Color) -> bool {
        self.move_changed_castling_rights[color as usize][0] < 0
    }

    pub fn can_castle_kingside(&self, color: Color) -> bool {
        self.move_changed_castling_rights[color as usize][1] < 0
    }

}