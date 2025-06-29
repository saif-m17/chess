use crate::board::*; 

#[derive(Debug, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveType {
    Normal,
    Castle { kingside: bool },
    EnPassant,
    DoublePawnPush,
    Promotion { piece: PieceType },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub from: u8,                    // 0-63 square index
    pub to: u8,                      // 0-63 square index  
    pub piece: PieceType,            // What piece is moving
    pub color: Color,                // What color is moving
    pub captured: Option<PieceType>, // What piece was captured (if any)
    pub move_type: MoveType,         // Special move information
}

impl Move {
    /// Creates a normal move (most common case)
    pub fn new_normal(from: u8, to: u8, piece: PieceType, color: Color, captured: Option<PieceType>) -> Self {
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
            (Color::White, true) => (4, 6),   // e1 to g1
            (Color::White, false) => (4, 2),  // e1 to c1
            (Color::Black, true) => (60, 62), // e8 to g8
            (Color::Black, false) => (60, 58), // e8 to c8
        };
        
        Move {
            from,
            to,
            piece: PieceType::King,
            color,
            captured: None,
            move_type: MoveType::Castle { kingside },
        }
    }
    
    /// Creates a pawn promotion move
    pub fn new_promotion(from: u8, to: u8, color: Color, captured: Option<PieceType>, promote_to: PieceType) -> Self {
        Move {
            from,
            to,
            piece: PieceType::Pawn,
            color,
            captured,
            move_type: MoveType::Promotion { piece: promote_to },
        }
    }
    
    /// Creates an en passant capture
    pub fn new_en_passant(from: u8, to: u8, color: Color) -> Self {
        Move {
            from,
            to,
            piece: PieceType::Pawn,
            color,
            captured: Some(PieceType::Pawn), // Always captures a pawn
            move_type: MoveType::EnPassant,
        }
    }
    
    /// Creates a double pawn push
    pub fn new_double_pawn_push(from: u8, to: u8, color: Color) -> Self {
        Move {
            from,
            to,
            piece: PieceType::Pawn,
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