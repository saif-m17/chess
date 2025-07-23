use num_enum::TryFromPrimitive;
use std::fmt;
use crate::utils::{*}; 
use crate::bitboards::{*}; 

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub fn opposite_color(&self) -> Color {
        [Color::Black, Color::White][*self as usize]
    }
    
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MoveType {
    Normal,
    Castle { kingside: bool },
    EnPassant,
    DoublePawnPush,
    Promotion { piece: Piece },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive)]
#[repr(u64)]
pub enum Square {
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2,    B2, C2, D2, E2, F2, G2, H2,
    A3,    B3, C3, D3, E3, F3, G3, H3,
    A4,    B4, C4, D4, E4, F4, G4, H4,
    A5,    B5, C5, D5, E5, F5, G5, H5,
    A6,    B6, C6, D6, E6, F6, G6, H6,
    A7,    B7, C7, D7, E7, F7, G7, H7,
    A8,    B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub const fn to_bitboard(self) -> u64 {
        1u64 << (self as u64)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Move {
    pub from: Square,                    // 0-63 square index
    pub to: Square,                      // 0-63 square index  
    pub piece: Piece,            // What piece is moving
    pub color: Color,                // What color is moving
    pub captured: Option<Piece>, // What piece was captured (if any)
    pub move_type: MoveType,         // Special move information
}

impl Move {

    /// Default move
    pub fn default() -> Self {
        Move {
            from: Square::A1,
            to: Square::A2,
            piece: Piece::Pawn,
            color: Color::White,
            captured: None,
            move_type: MoveType::Normal,
        }
    }

    /// Creates a normal move (most common case)
    pub fn new_normal(from: Square, to: Square, piece: Piece, color: Color, captured: Option<Piece>) -> Self {
        Move {
            from,
            to,
            piece,
            color,
            captured,
            move_type: MoveType::Normal,
        }
    }
    
    /// Creates a castling move
    pub fn new_castle(color: Color, kingside: bool) -> Self {
        let (from, to) = match (color.clone(), kingside) {
            (Color::White, true) => (Square::E1, Square::G1),  
            (Color::White, false) => (Square::E1, Square::C1),  
            (Color::Black, true) => (Square::E8, Square::G8), 
            (Color::Black, false) => (Square::E8, Square::C8), 
        };
        
        Move {
            from,
            to,
            piece: Piece::King,
            color,
            captured: None,
            move_type: MoveType::Castle { kingside },
        }
    }
    
    /// Creates a pawn promotion move
    pub fn new_promotion(from: Square, to: Square, color: Color, captured: Option<Piece>, promote_to: Piece) -> Self {
        Move {
            from,
            to,
            piece: Piece::Pawn,
            color,
            captured,
            move_type: MoveType::Promotion { piece: promote_to },
        }
    }
    
    /// Creates an en passant capture
    pub fn new_en_passant(from: Square, to: Square, color: Color) -> Self {
        Move {
            from,
            to,
            piece: Piece::Pawn,
            color,
            captured: Some(Piece::Pawn), // Always captures a pawn
            move_type: MoveType::EnPassant,
        }
    }
    
    /// Creates a double pawn push
    pub fn new_double_pawn_push(from: Square, to: Square, color: Color) -> Self {
        Move {
            from,
            to,
            piece: Piece::Pawn,
            color,
            captured: None,
            move_type: MoveType::DoublePawnPush,
        }
    }
    
    /// Check if this move is a capture
    pub fn is_capture(&self) -> bool {
        self.captured.is_some()
    }
    
    /// Check if this move is a promotion
    pub fn is_promotion(&self) -> bool {
        matches!(self.move_type, MoveType::Promotion { .. })
    }
    
    /// Check if this move is castling
    pub fn is_castle(&self) -> bool {
        matches!(self.move_type, MoveType::Castle { .. })
    }

    /// Encode move to action # 
    pub fn encode_action(&self) -> u32 {
        let from_index = self.from as u32;
        let to_index = self.to as u32;

        let moving_piece = match self.piece {
            Piece::Pawn => 0,
            Piece::Knight => 1,
            Piece::Bishop => 2,
            Piece::Rook => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        };

        let promo_piece = match self.move_type {
            MoveType::Promotion { piece } => {
                match piece {
                    Piece::Knight => 1,
                    Piece::Bishop => 2,
                    Piece::Rook => 3,
                    Piece::Queen => 4,
                    _ => 0,
                }
            },
            _ => 0,
        } as u32; 

        let move_type = match self.move_type {
            MoveType::Promotion { piece: _ } => 0,
            MoveType::Normal => 0,
            MoveType::Castle {kingside: _ } => 1,
            MoveType::EnPassant => 2,
            MoveType::DoublePawnPush => 3,
        } as u32;

        let kingside = match self.move_type {
            MoveType::Castle { kingside } => {
                if kingside {1} else {0}
            },
            _ => 0,
        }; 

        let captured = match self.captured {
            Some(Piece::Pawn) => 0,
            Some(Piece::Knight) => 1,
            Some(Piece::Bishop) => 2,
            Some(Piece::Rook) => 3,
            Some(Piece::Queen) => 4,
            Some(Piece::King) => 5,
            _ => 6,
        }; 

        let color = self.color as u32; 

        (from_index) | (to_index << 6) | (moving_piece << 12) | (promo_piece << 15) | (move_type << 18) | (kingside << 20) |
        (captured << 21) | (color << 24) 

    }

    /// Decode action # to move. 
    pub fn decode_action(action: u32) -> Self {
        let from_index = action & FROM_MASK;
        let to_index = action & TO_MASK;

        let from = Square::try_from(from_index as u64).unwrap();
        let to = Square::try_from(to_index as u64).unwrap();

        let moving_piece = match action & PIECE_MASK {
            0 => Some(Piece::Pawn),
            1 => Some(Piece::Knight),
            2 => Some(Piece::Bishop),
            3 => Some(Piece::Rook),
            4 => Some(Piece::Queen),
            _ => Some(Piece::King),
        }; 

        let promo_piece: Option<Piece> = match action & PROMO_PIECE_MASK {
            1 => Some(Piece::Knight),
            2 => Some(Piece::Bishop),
            3 => Some(Piece::Rook),
            4 => Some(Piece::Queen),
            _ => None, 
        }; 

        let kingside_bool = match action & KINGSIDE_MASK {
            1 => true,
            _ => false,
        }; 

        let move_type: MoveType = match action & MOVE_TYPE_MASK {
            0 => {
                if promo_piece.is_some() { MoveType::Promotion { piece: promo_piece.unwrap() }} else { MoveType::Normal}
            },
            1 => MoveType::Castle {kingside: kingside_bool},
            2 => MoveType::EnPassant,
            _ => MoveType::DoublePawnPush,
        }; 

        let captured = match action & CAPTURED_MASK {
            0 => Some(Piece::Pawn),
            1 => Some(Piece::Knight),
            2 => Some(Piece::Bishop),
            3 => Some(Piece::Rook),
            4 => Some(Piece::Queen),
            5 => Some(Piece::King),
            _ => None,
        }; 

        let color = match action & COLOR_MASK {
            0 => Color::White,
            _ => Color::Black,
        }; 

        match move_type {
            MoveType::Normal => Move::new_normal(from, to, moving_piece.unwrap(), color, captured),
            MoveType::Castle { kingside } => Move::new_castle(color, kingside),
            MoveType::DoublePawnPush => Move::new_double_pawn_push(from, to, color),
            MoveType::EnPassant => Move::new_en_passant(from, to, color),
            MoveType::Promotion { piece } => Move::new_promotion(from, to, color, captured, piece)
        }

    }

}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from = square_to_string(self.from as u8);
        let to = square_to_string(self.to as u8);
        let mut move_str = format!("{}{}", from, to);

        if let MoveType::Promotion {piece} = self.move_type {
                let promo_char = match piece {
                    Piece::Knight => 'n',
                    Piece::Bishop => 'b',
                    Piece::Rook   => 'r',
                    Piece::Queen  => 'q',
                    _ => panic!("Invalid promotion piece"),
                };
                move_str.push(promo_char)
        }

        write!(f, "{}", move_str)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North = 0, South, East, West,
    NorthEast, NorthWest, SouthEast, SouthWest,
}

pub const DIRECTION_COORDINATES: [(i8, i8); 8] = [
    (1, 0), // North
    (-1, 0), // South
    (0, 1), // East
    (0, -1), //West
    (1, 1), // Northeast
    (1, -1), // Northwest
    (-1, 1), // Southeast
    (-1, -1), // Southwest
]; 