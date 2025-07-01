use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Empty,
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

#[derive(Debug, Clone, PartialEq)]
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
    pub fn to_bitboard(self) -> u64 {
        1u64 << (self as u64)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub from: Square,                    // 0-63 square index
    pub to: Square,                      // 0-63 square index  
    pub piece: Piece,            // What piece is moving
    pub color: Color,                // What color is moving
    pub captured: Option<Piece>, // What piece was captured (if any)
    pub move_type: MoveType,         // Special move information
}

impl Move {
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
            (Color::White, true) => (Square::E1, Square::G1),   // e1 to g1
            (Color::White, false) => (Square::E1, Square::C1),  // e1 to c1
            (Color::Black, true) => (Square::E8, Square::G8), // e8 to g8
            (Color::Black, false) => (Square::E8, Square::C8), // e8 to c8
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
}