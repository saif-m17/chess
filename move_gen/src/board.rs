use crate::bitboards::{*}; 
use crate::moves::{Color::{self, *}, Move, MoveType, Piece::{self, *}, Square::{self, *}};

#[derive(Clone)]
pub struct Board {
    pub pieces: [[Bitboard; 6]; 2],
    pub piece_lookup: [Option<Piece>; 64],
    pub past_moves: Vec<Move>,
    pub en_passant_square: Option<Square>,
    pub castling_rights: [[bool; 2]; 2],
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
        pieces[Black as usize][Pawn as usize] = RANK_2;
        pieces[Black as usize][Knight as usize] = B1.to_bitboard() | G1.to_bitboard();
        pieces[Black as usize][Bishop as usize] = C1.to_bitboard() | F1.to_bitboard(); 
        pieces[Black as usize][Rook as usize] = A1.to_bitboard() | H1.to_bitboard();
        pieces[Black as usize][King as usize] = E1.to_bitboard();
        pieces[Black as usize][Queen as usize] = D1.to_bitboard();

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

        let castling_rights = [[true, true], [true, true]]; 

        Board { pieces, piece_lookup, past_moves, en_passant_square:None, castling_rights }
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
        match mve.move_type {
            MoveType::Normal => self.make_normal_move(mve),
            MoveType::Castle { kingside } => self.make_castle_move(mve, kingside),
            MoveType::EnPassant => self.make_en_passant_move(mve),
            MoveType::DoublePawnPush => self.make_double_push_move(mve),
            MoveType::Promotion { piece } => self.make_promotion_move(mve, piece),
        }
    }

    pub fn get_en_passant(&self) -> Option<Bitboard> {
        self.en_passant_square.map(Bitboard::from_square)
    }

    fn make_normal_move(&mut self, mve: Move) {
        self.pieces[mve.color as usize][mve.piece as usize].clear_bit(mve.from as u64); 
        self.pieces[mve.color as usize][mve.piece as usize].set_bit(mve.to as u64); 

        if let Some(captured_piece) = mve.captured {
            self.pieces[mve.color.opposite_color() as usize][captured_piece as usize].clear_bit(mve.to as u64); 
        }

        if mve.piece == Rook && mve.from == ROOK_CASTLING_INITIAL_SQUARE[mve.color as usize][0] && self.castling_rights[mve.color as usize][0] {
            self.castling_rights[mve.color as usize][0] = false; 
        } else if mve.piece == Rook && mve.from == ROOK_CASTLING_INITIAL_SQUARE[mve.color as usize][1] && self.castling_rights[mve.color as usize][1] {
            self.castling_rights[mve.color as usize][1] = false; 
        }

        self.piece_lookup[mve.from as usize] = None;
        self.piece_lookup[mve.to as usize] = Some(mve.piece); 

        self.past_moves.push(mve); 
    }

    fn make_castle_move(&mut self, mve: Move, kingside: bool) {
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

        self.castling_rights = [[false, false], [false, false]]; 

        self.past_moves.push(mve); 
    }

    fn make_double_push_move(&mut self, mve: Move) {
        self.pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        self.pieces[mve.color as usize][Pawn as usize].set_bit(mve.to as u64);

        self.piece_lookup[mve.from as usize] = None;
        self.piece_lookup[mve.to as usize] = Some(Pawn);

        let to_index = mve.to as u8;
        let offset = OFFSET_SINGLE_PUSH[mve.color as usize] as i16;
        let en_passant_index =  (to_index as i16 - offset) as u64;

        self.en_passant_square = Some(Square::try_from(en_passant_index).unwrap()); 

    }

    fn make_en_passant_move(&mut self, mve: Move) {
        self.pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        self.pieces[mve.color as usize][Pawn as usize].set_bit(mve.to as u64); 

        self.piece_lookup[mve.from as usize] = None;
        self.piece_lookup[mve.to as usize] = Some(Pawn); 

        let to_index = mve.to as u8;
        let offset = OFFSET_SINGLE_PUSH[mve.color as usize] as i16;
        let captured_piece_index = (to_index as i16 - offset) as u64;

        self.pieces[mve.color.opposite_color() as usize][Pawn as usize].clear_bit(captured_piece_index);
        self.piece_lookup[captured_piece_index as usize] = None;
    }

    fn make_promotion_move(&mut self, mve: Move, promotion: Piece) {
        self.pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        self.pieces[mve.color as usize][promotion as usize].set_bit(mve.to as u64); 

        if let Some(captured_piece) = mve.captured {
            self.pieces[mve.color.opposite_color() as usize][captured_piece as usize].clear_bit(mve.to as u64); 
        }

        self.piece_lookup[mve.from as usize] = None;
        self.piece_lookup[mve.to as usize] = Some(Pawn); 

    }

    pub fn make_shallow_move(&self, mve: Move) -> [[Bitboard; 6]; 2] {
        match mve.move_type {
            MoveType::Normal => self.make_shallow_normal_move(mve),
            MoveType::Castle { kingside } => self.make_shallow_castle_move(mve, kingside),
            MoveType::EnPassant => self.make_shallow_en_passant_move(mve),
            MoveType::DoublePawnPush => self.make_shallow_double_push_move(mve),
            MoveType::Promotion { piece } => self.make_shallow_promotion_move(mve, piece),
        }
    }

    fn make_shallow_normal_move(&self, mve: Move) -> [[Bitboard; 6]; 2] {
        let mut copied_pieces = self.pieces.clone(); 
        copied_pieces[mve.color as usize][mve.piece as usize].clear_bit(mve.from as u64); 
        copied_pieces[mve.color as usize][mve.piece as usize].set_bit(mve.to as u64); 
        if let Some(captured_piece) = mve.captured {
            copied_pieces[mve.color.opposite_color() as usize][captured_piece as usize].clear_bit(mve.to as u64); 
        }
        copied_pieces
    }

    fn make_shallow_castle_move(&self, mve: Move, kingside: bool) -> [[Bitboard; 6]; 2] {
        let mut copied_pieces = self.pieces.clone(); 
        copied_pieces[mve.color as usize][King as usize].clear_bit(mve.from as u64);
        copied_pieces[mve.color as usize][King as usize].set_bit(mve.to as u64);

        let rook_to_index = ROOK_CASTLING_DIRECTION[kingside as usize](Bitboard::from_square(mve.to)).trailing_zeros() as u64; 
        let rook_from_index = ROOK_CASTLING_INITIAL_SQUARE[mve.color as usize][kingside as usize] as u64;

        copied_pieces[mve.color as usize][Rook as usize].clear_bit(rook_from_index);
        copied_pieces[mve.color as usize][Rook as usize].set_bit(rook_to_index); 
        copied_pieces
    }

    fn make_shallow_double_push_move(&self, mve: Move) -> [[Bitboard; 6]; 2] {
        let mut copied_pieces = self.pieces.clone(); 
        copied_pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        copied_pieces[mve.color as usize][Pawn as usize].set_bit(mve.to as u64); 
        copied_pieces
        
    }

    fn make_shallow_en_passant_move(&self, mve: Move) -> [[Bitboard; 6]; 2] {
        let mut copied_pieces = self.pieces.clone(); 
        copied_pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        copied_pieces[mve.color as usize][Pawn as usize].set_bit(mve.to as u64); 

        let to_index = mve.to as u8;
        let offset = OFFSET_SINGLE_PUSH[mve.color as usize] as i16;
        let captured_piece_index = (to_index as i16 - offset) as u64;

        copied_pieces[mve.color.opposite_color() as usize][Pawn as usize].clear_bit(captured_piece_index);
        copied_pieces

    }

    fn make_shallow_promotion_move(&self, mve: Move, promotion: Piece) -> [[Bitboard; 6]; 2] {
        let mut copied_pieces = self.pieces.clone(); 
        copied_pieces[mve.color as usize][Pawn as usize].clear_bit(mve.from as u64);
        copied_pieces[mve.color as usize][promotion as usize].set_bit(mve.to as u64); 

        if let Some(captured_piece) = mve.captured {
            copied_pieces[mve.color.opposite_color() as usize][captured_piece as usize].clear_bit(mve.to as u64); 
        }
        copied_pieces
    }

}