use std::collections::HashMap;
use crate::GameState;
use crate::action_space::{Action, ActionID}; 

const PUCB_C: f32 = 1.0; // c in upper confidence bound calculation for MCTS -> higher encourages exploration

pub struct Node {
    // cloning board state at subsequent nodes in the tree for now, try to just replay moves instead later
    game: GameState, 
    zobrist_hash: u64, // game hash
    prior: HashMap<ActionID, f32>,
    value: HashMap<ActionID, f32>,
    visit_count: HashMap<ActionID, u32>,
    children: HashMap<ActionID, u64>, // zobrist hashes of child nodes 
    parent: Option<u64>, // parent hash - None if root
    legal_actions: Vec<Action>,
    total_visit_count: u32,
}

impl Node {
    pub fn game(&self) -> &GameState {&self.game}
    pub fn prior(&self, action: ActionID) -> f32 {*self.prior.get(&action).unwrap_or(&0.0)}
    pub fn value(&self, action: ActionID) -> f32 {*self.value.get(&action).unwrap_or(&0.0)}
    pub fn visit_count(&self, action: ActionID) -> u32 {*self.visit_count.get(&action).unwrap_or(&0)}
    pub fn children(&self) -> &HashMap<ActionID, u64> {&self.children}
    pub fn zhash(&self) -> u64 {self.zobrist_hash}
    pub fn legal_actions(&self) -> &Vec<Action> {&self.legal_actions}
    pub fn total_visit_count(&self) -> u32 {self.total_visit_count}
    pub fn parent(&self) -> Option<u64> {self.parent}

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
        let total_visit_count = 0; 
        let parent = None; 

        Node {
            game, prior, value, visit_count, children, parent, zobrist_hash, legal_actions, total_visit_count,
        }
    }

    pub fn from_game_state(mut game: GameState, parent: Option<u64>) -> Self {
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
        let total_visit_count = 0; 

        Node {
            game, prior, value, visit_count, children, parent, zobrist_hash, legal_actions, total_visit_count,
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
        let total_visit_count = 0;
        let parent = None; 

        Node {
            game, prior, value, visit_count, children, parent, zobrist_hash, legal_actions, total_visit_count, 
        }
    }

    pub fn select_child(&mut self) -> (Option<u64>, Option<Node>) {
        let best_action = self.legal_actions()
            .iter()
            .max_by(|a, b|{
                let sqrt_total_visits = (self.total_visit_count() as f32).sqrt();
                let ucba = self.value(a.action_id()) + PUCB_C * self.prior(a.action_id()) * sqrt_total_visits / (1.0 + (self.visit_count(a.action_id()) as f32)); 
                let ucbb = self.value(b.action_id()) + PUCB_C * self.prior(b.action_id()) * sqrt_total_visits / (1.0 + (self.visit_count(b.action_id()) as f32));
                ucba.partial_cmp(&ucbb).unwrap()
            })
            .unwrap(); 
        
        if self.children.contains_key(&best_action.action_id()) {
            return (Some(*self.children.get(&best_action.action_id()).unwrap()), None); 
        } else {
            let mut game = self.game().clone(); 
            if game.make_move(*best_action.get_move()).is_err() {
                return (None, None)
            }; 
            let hash = game.current_hash(); 
            self.children.insert(best_action.action_id(), hash); 
            let node = Node::from_game_state(game, Some(self.zhash())); 
            return (Some(hash), Some(node))
        }
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