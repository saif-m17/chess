use crate::bitboards::BitboardExt;
use crate::moves::{Move, Piece, Square, Color};
use crate::game_state::GameState; 
use crate::bitboards::Bitboard;

pub const SCHEMA_VERSION: u32 = 0;
pub const BOARD_SIZE: u32 = 64;
pub const EMPTY_OR_FULL: [Bitboard; 2] = [0u64, u64::MAX]; // all 0s if false, or 1s if True 

pub struct FeatureSchema {
    channels: usize,
    channel_names: Vec<&'static str>,
}

impl FeatureSchema {
    pub fn new() -> FeatureSchema {
        let channels: usize = 20; 
        let channel_names = vec![
            "white_pawns", "white_knights", "white_bishops", "white_rooks",
            "white_queen", "white_king", "black_pawns", "black_knights", 
            "black_bishops", "black_rooks", "black_queen", "black_king",
            "side_to_move", "white_queenside", "white_kingside", "black_queenside",
            "black_kingside", "en_passant_sq", "halfmove_clock", "threefold_count",
        ];
        FeatureSchema { channels, channel_names }
    }
    pub fn channels(&self) -> usize { self.channels }

    pub fn shape(&self) -> (usize, usize, usize) { (self.channels, 8, 8) }

    pub fn index_of(&self, name: &str) -> Option<usize> { self.channel_names.iter().position(|&p| p == name) }

    pub fn channel_names(&self) -> &[&'static str] {&self.channel_names}
    
}

pub struct TensorBuffer {
    buf: Vec<f32>,
    schema: FeatureSchema,
}

impl TensorBuffer {
    pub fn new(schema: FeatureSchema) -> TensorBuffer {
        TensorBuffer {
            buf: vec![0.0f32; schema.channels() * 64],
            schema: schema,
        }
    }

    pub fn len(&self) -> usize {
        self.schema.channels() * 64
    }

    pub fn clear(&mut self) {
        self.buf.fill(0.0f32)
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.buf
    }

    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        &mut self.buf
    }

    pub fn into_inner(self) -> Vec<f32> {
        self.buf
    }

    pub fn write_from(&mut self, game: &GameState) {
        let board = game.get_board(); 
        for &name in self.schema.channel_names().iter() {
            let index = self.schema.index_of(name).expect("channel not found");
            match name {
                "white_pawns" => {
                    let bb = board.get_pawn_bb(Color::White);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "black_pawns" => {
                    let bb = board.get_pawn_bb(Color::Black);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "white_knights" => {
                    let bb = board.get_knight_bb(Color::White);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "black_knights" => {
                    let bb = board.get_knight_bb(Color::Black);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "white_bishops" => {
                    let bb = board.get_bishop_bb(Color::White);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "black_bishops" => {
                    let bb = board.get_bishop_bb(Color::Black);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "white_rooks" => {
                    let bb = board.get_rook_bb(Color::White);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "black_rooks" => {
                    let bb = board.get_rook_bb(Color::Black);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                }
                "white_queen" => {
                    let bb = board.get_queen_bb(Color::White);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "black_queen" => {
                    let bb = board.get_queen_bb(Color::Black);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                }
                "white_king" => {
                    let bb = board.get_king_bb(Color::White);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "black_king" => {
                    let bb = board.get_king_bb(Color::Black);
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                }
                "side_to_move" => {
                    let bb = EMPTY_OR_FULL[game.get_side() as usize];
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "white_queenside" => {
                    let bb = EMPTY_OR_FULL[board.can_castle_queenside(Color::White) as usize];
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                }
                "white_kingside" => {
                    let bb = EMPTY_OR_FULL[board.can_castle_kingside(Color::Black) as usize];
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                }
                "black_queenside" => {
                    let bb = EMPTY_OR_FULL[board.can_castle_queenside(Color::White) as usize];
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "black_kingside" => {
                    let bb = EMPTY_OR_FULL[board.can_castle_kingside(Color::Black) as usize];
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                }
                "en_passant_sq" => {
                    let bb = if let Some(sq) = board.get_en_passant_square() {
                        sq.to_bitboard()
                    } else {
                        0u64
                    }; 
                    TensorBuffer::write_plane(bb, index, &mut self.buf);
                },
                "half_move_clock" => {
                    let val: f32 = (game.get_half_move_clock() as f32) / 100.0; 
                    TensorBuffer::write_scalar_plane(val, index, &mut self.buf);
                },
                "threefold_count" => {
                    let val: f32 = (game.get_three_fold_count() as f32) / 3.0;
                    TensorBuffer::write_scalar_plane(val, index, &mut self.buf);
                },
                _ => (),
            }; 
            
        }
    }

    fn write_plane(mut bb: Bitboard, index: usize, plane: &mut Vec<f32>) { 
        while bb != 0 {
            let sq = bb.trailing_zeros() as usize; 
            bb.clear_bit(sq as u64);
            plane[index * 64 + sq] = 1.0; 
        }

    }

    fn write_scalar_plane(val: f32, index: usize, plane: &mut Vec<f32>) {
        for i in 0..64 {
            plane[index * 64 + i] = val; 
        }
    }

}