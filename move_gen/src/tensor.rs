use crate::moves::{Move, Piece, Square, Color};
use crate::game_state::GameState; 

pub const SCHEMA_VERSION: u32 = 0;
pub const BOARD_SIZE: u32 = 64;

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
        for name in self.schema.channel_names().iter() {
            match *name {
                "white_pawns" => [[0.0f32; 8]; 8],
                _ => [[0.0f32; 8]; 8],
            }; 
        }
    }

}