use std::collections::HashMap;
use crate::utils::MoveList;
use crate::GameState;
use crate::action_space::{encode_action, num_actions, Action, ActionID}; 

pub struct Node {
    game: GameState,
    zobrist_hash: u64, // game hash
    prior: HashMap<ActionID, f32>,
    value: HashMap<ActionID, f32>,
    visit_count: HashMap<ActionID, u32>,
    children: HashMap<ActionID, u64>, // zobrist hashes of child nodes 
    legal_actions: Vec<Action>,
}

impl Node {
    pub fn game(&self) -> &GameState {&self.game}
    pub fn prior(&self, action: ActionID) -> f32 {*self.prior.get(&action).unwrap_or(&0.0)}
    pub fn value(&self, action: ActionID) -> f32 {*self.value.get(&action).unwrap_or(&0.0)}
    pub fn visit_count(&self, action: ActionID) -> u32 {*self.visit_count.get(&action).unwrap_or(&0)}
    pub fn children(&self) -> &HashMap<ActionID, u64> {&self.children}
    pub fn zhash(&self) -> u64 {self.zobrist_hash}
    pub fn legal_actions(&self) -> &Vec<Action> {&self.legal_actions}

    pub fn new_default() -> Self {
        let mut game = GameState::new();
        let move_list = game.get_gamestate_legal_moves(); 

        let mut prior = HashMap::new();
        let mut value = HashMap::new();
        let mut visit_count = HashMap::new();
        let children = HashMap::new(); 
        let mut legal_actions = Vec::new(); 

        let uniform_prior = 1.0 / (move_list.len() as f32); 

        for mv in move_list.iter() {
            let action = Action::new(mv); 
            prior.insert(action.action_id(), uniform_prior);
            value.insert(action.action_id(), 0.0);
            visit_count.insert(action.action_id(), 0);
            legal_actions.push(action); 
             
        }

        let zobrist_hash = game.current_hash(); 

        Node {
            game, prior, value, visit_count, children, zobrist_hash, legal_actions,
        }
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut game = GameState::from_fen(fen).unwrap();
        let moves_list = game.get_gamestate_legal_moves(); 

        
        let mut prior = HashMap::new();
        let mut value = HashMap::new();
        let mut visit_count = HashMap::new();
        let children = HashMap::new(); 
        let mut legal_actions = Vec::with_capacity(moves_list.len()); 

        let uniform_prior = 1.0 / (moves_list.len() as f32); 

        for mv in moves_list.iter() {
            let action = Action::new(mv); 
            prior.insert(action.action_id(), uniform_prior);
            value.insert(action.action_id(), 0.0);
            visit_count.insert(action.action_id(), 0);
            legal_actions.push(action); 
        }

        let zobrist_hash = game.current_hash(); 
        
        Node {
            game, prior, value, visit_count, children, zobrist_hash, legal_actions
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