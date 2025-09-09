use std::collections::HashMap;
use crate::GameState;
use crate::action_space::num_actions; 

pub struct Node {
    game: GameState,
    zobrist_hash: u64, // game hash
    prior: [f32; num_actions() as usize],
    value: [f32; num_actions() as usize],
    visit_count: [u32; num_actions() as usize],
    children: Vec<u64>, // zobrist hashes of child nodes 
}

impl Node {
    pub fn game(&self) -> &GameState {&self.game}
    pub fn prior(&self) -> [f32; num_actions() as usize] {self.prior}
    pub fn value(&self) -> [f32; num_actions() as usize] {self.value}
    pub fn visit_count(&self) -> [u32; num_actions() as usize] {self.visit_count}
    pub fn children(&self) -> &Vec<u64> {&self.children}
    pub fn zhash(&self) -> u64 {self.zobrist_hash}

    pub fn new_default() -> Self {
        let game = GameState::new();
        let zobrist_hash = game.current_hash(); 
        let prior = [1.0f32 / (num_actions() as f32); num_actions() as usize];
        let value = [0.0f32; num_actions() as usize];
        let visit_count = [0u32; num_actions() as usize];
        let children: Vec<u64> = Vec::new(); 
        Node {
            game, prior, value, visit_count, children, zobrist_hash,
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        let game = GameState::from_fen(fen).unwrap();
        let zobrist_hash = game.current_hash(); 
        let prior = [1.0f32 / (num_actions() as f32); num_actions() as usize];
        let value = [0.0f32; num_actions() as usize];
        let visit_count = [0u32; num_actions() as usize];
        let children: Vec<u64> = Vec::new(); 
        Node {
            game, prior, value, visit_count, children, zobrist_hash,
        }
    }

    pub fn select_child(&self) -> u64 {
        // upper confidence bound calculation
        todo!()
    }

}
pub struct MCTS {
    tree: HashMap<u64, Node>, // Zobrist hash to node corresponding to that gamestate
    root: u64, // zobrist hash of root

}

impl MCTS {
    pub fn tree(&self) -> &HashMap<u64, Node> {&self.tree}
    pub fn root(&self) -> u64 {self.root}

    pub fn new_default() -> Self {
        let root_node = Node::new_default(); 
        MCTS {
            tree: HashMap::new(),
            root: root_node.zhash(),
        }
    }
}