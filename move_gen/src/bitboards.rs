use crate::moves::{Square, Direction}; 
use crate::attacktables::RAYS; 

// Rank constants
pub const RANK_1: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_11111111;
pub const RANK_2: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
pub const RANK_3: u64 = 0b00000000_00000000_00000000_00000000_00000000_11111111_00000000_00000000;
pub const RANK_4: u64 = 0b00000000_00000000_00000000_00000000_11111111_00000000_00000000_00000000;
pub const RANK_5: u64 = 0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000;
pub const RANK_6: u64 = 0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000;
pub const RANK_7: u64 = 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
pub const RANK_8: u64 = 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000;

// File constants
pub const FILE_A: u64 = 0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001;
pub const FILE_B: u64 = 0b00000010_00000010_00000010_00000010_00000010_00000010_00000010_00000010;
pub const FILE_C: u64 = 0b00000100_00000100_00000100_00000100_00000100_00000100_00000100_00000100;
pub const FILE_D: u64 = 0b00001000_00001000_00001000_00001000_00001000_00001000_00001000_00001000;
pub const FILE_E: u64 = 0b00010000_00010000_00010000_00010000_00010000_00010000_00010000_00010000;
pub const FILE_F: u64 = 0b00100000_00100000_00100000_00100000_00100000_00100000_00100000_00100000;
pub const FILE_G: u64 = 0b01000000_01000000_01000000_01000000_01000000_01000000_01000000_01000000;
pub const FILE_H: u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000;

// Empty and full board
pub const EMPTY: u64 = 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
pub const FULL: u64 = 0b11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111;

// Some helpful functions for manipulating bitboards
pub type Bitboard = u64; 

pub trait BitboardExt {
    fn clear_bit(&mut self, index: u64);
    fn set_bit(&mut self, index: u64);
    fn get_bit(self, index: u64) -> bool;
    fn shift_north(&mut self);
    fn shift_south(&mut self);
    fn shift_east(&mut self);
    fn shift_west(&mut self);
    fn from_square(square: Square) -> Self; 
    fn relevant_bits(self, square: Square, direction: Direction) -> usize;
    fn display(self); 
    fn to_string(self) -> String; 
}

impl BitboardExt for Bitboard {
    fn clear_bit(&mut self, index: u64) { *self &= !(1u64 << index);  }
    fn set_bit(&mut self, index: u64) { *self |= 1u64 << index; }
    fn get_bit(self, index: u64) -> bool { (self >> index) & 1 == 1 }

    fn shift_north(&mut self) { *self = (*self & !RANK_8) << 8 }
    fn shift_south(&mut self)  { *self = (*self & !RANK_1) >> 8 }
    fn shift_west(&mut self) { *self = (*self & !FILE_H) >> 1 }
    fn shift_east(&mut self) { *self = (*self & !FILE_A) << 1 }

    fn from_square(square: Square) -> Self { 1u64 << (square as u64) }

    fn relevant_bits(self, square: Square, direction: Direction) -> usize {
        let ray = RAYS[square as usize][direction as usize];
        let relevant_blockers = self & ray;
        relevant_blockers.count_ones() as usize 
    }

    fn display(self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = (self >> square) & 1;
                print!("{} ", bit);
            }
            println!();
        }
        println!("0x{:016x}", self);
    }

    fn to_string(self) -> String {
        let mut board_str = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = 1u64 << square;
                if self & bit != 0 {
                    board_str.push('1');
                } else {
                    board_str.push('.');
                }
            }
            board_str.push('\n');
        }
        board_str

    }    
}

// Free functions for shifting pawns
pub fn shift_north(bb: Bitboard) -> Bitboard { (bb & !RANK_8) << 8 }
pub fn shift_south(bb: Bitboard) -> Bitboard { (bb & !RANK_1) >> 8 }
pub fn shift_east(bb: Bitboard) -> Bitboard { (bb & !FILE_A) << 1 }
pub fn shift_west(bb: Bitboard) -> Bitboard { (bb & !FILE_H) >> 1 }

// Color-relative movement
pub const FORWARD_SHIFT: [fn(Bitboard) -> Bitboard; 2] = [shift_north, shift_south];
pub const PAWN_DOUBLE_RANK: [Bitboard; 2] = [RANK_3, RANK_6]; // one more than it should be, since we'll use already pushed pawns
pub const PAWN_PROMOTION_RANK: [Bitboard; 2] = [RANK_7, RANK_2]; //one less than it should be, check before we make the move
pub const OFFSET_SINGLE_PUSH: [i8; 2] = [8, -8];
pub const OFFSET_DOUBLE_PUSH: [i8; 2] = [16, -16]; 

// Pawn Attacks - Fix check file first then shift 
pub const fn pawn_attack_left_white(bb: Bitboard) -> Bitboard { (bb & !FILE_H) << 7 }
pub const fn pawn_attack_right_white(bb: Bitboard) -> Bitboard { (bb& !FILE_A) << 9 }
pub const fn pawn_attack_left_black(bb: Bitboard) -> Bitboard { (bb & !FILE_H) >> 9 }
pub const fn pawn_attack_right_black(bb: Bitboard) -> Bitboard { (bb & !FILE_A) >> 7}

pub const PAWN_ATTACK_LEFT: [fn(Bitboard) -> Bitboard; 2] = [
    pawn_attack_left_white,
    pawn_attack_left_black,
];

pub const PAWN_ATTACK_RIGHT: [fn(Bitboard) -> Bitboard; 2] = [
    pawn_attack_right_white,
    pawn_attack_right_black,
];

// Pawn promotions
pub fn pawn_promotion_white(bb: Bitboard) -> Bitboard {
    (bb & RANK_7) << 8
}

pub fn pawn_promotion_black(bb: Bitboard) -> Bitboard {
    (bb & RANK_2) >> 8
}

pub const PAWN_PROMOTION: [fn(Bitboard) -> Bitboard; 2] = [pawn_promotion_white, pawn_promotion_black]; 

// Knight attacks
pub const fn gen_knight_attacks(bb: Bitboard) -> Bitboard {
    let move1 = (bb & !RANK_8 & !FILE_G & !FILE_H) << 10;
    let move2 = (bb & !RANK_1 & !FILE_G & !FILE_H) >> 6;
    let move3 = (bb & !RANK_1 & !RANK_2 & !FILE_H) >> 15;
    let move4 = (bb & !RANK_7 & !RANK_8 & !FILE_H) << 17;
    let move5 = (bb & !RANK_8 & !FILE_A & !FILE_B) << 6;
    let move6 = (bb & !RANK_1 & !FILE_A & !FILE_B) >> 10;
    let move7 = (bb & !RANK_1 & !RANK_2 & !FILE_A) >> 17;
    let move8 = (bb & !RANK_7 & !RANK_8 & !FILE_A) << 15;

    move1 | move2 | move3 | move4 | move5 | move6 | move7 | move8
}

// King attacks
pub const fn gen_king_attacks(bb: Bitboard) -> Bitboard {
    let move1 = (bb & !RANK_8) << 8;
    let move2 = (bb & !RANK_8 & !FILE_H) << 7; 
    let move3 = (bb & !RANK_8 & !FILE_A) << 9;
    let move4 = (bb & !RANK_1) >> 8;
    let move5 = (bb & !RANK_1 & !FILE_H) >> 9; 
    let move6 = (bb & !RANK_1 & !FILE_A) >> 7; 
    let move7 = (bb & !FILE_H) << 1; 
    let move8 = (bb & !FILE_A) >> 1; 

    move1 | move2 | move3 | move4 | move5 | move6 | move7 | move8 
}

// Sliding Pieces related constants
pub const ROOK_DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
pub const BISHOP_DIRECTIONS: [Direction; 4] = [Direction::NorthEast, Direction::NorthWest, Direction::SouthEast, Direction::SouthWest];
pub const QUEEN_DIRECTIONS: [Direction; 8] = [
    Direction::North, Direction::South, Direction::East, Direction::West,
    Direction::NorthEast, Direction::NorthWest, Direction::SouthEast, Direction::SouthWest]; 

// Castling related constant functions
pub const ROOK_CASTLING_DIRECTION: [fn(Bitboard) -> Bitboard; 2] = [shift_east, shift_west]; // Queenside, Kingside
pub const ROOK_CASTLING_INITIAL_SQUARE: [[Square; 2]; 2] = [[Square::A1, Square::A8], [Square::H1, Square::H8]]; // White first, Queenside first

// Misc utility funtions

