use std::collections::HashMap;
use crate::weighted_sampler::WeightedSampler; 
use rand::rngs::ThreadRng;
use rand::Rng;
use rand::thread_rng; 
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
    parent: Option<(u64, ActionID)>, // parent hash - None if root
    legal_actions: Vec<Action>,
    unvisited_actions: WeightedSampler<Action>,
    total_visit_count: u32,
    rng: ThreadRng
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
    pub fn parent(&self) -> &Option<(u64, ActionID)> {&self.parent}

    pub fn fully_expanded(&self) -> bool {
        self.unvisited_actions.len() == 0
    }

    pub fn game_over(&self) -> bool {
        self.game.get_outcome().is_some()
    }

    /// Default MCTree Node. Begins with uniform prior for policy.
    pub fn new_default() -> Self {
        let mut game = GameState::new();
        let move_list = game.get_gamestate_legal_moves(); 

        let mut prior = HashMap::new();
        let mut value = HashMap::new();
        let mut visit_count = HashMap::new();
        let children = HashMap::new(); 
        let mut legal_actions = Vec::with_capacity(move_list.len()); 
        let mut unvisited_actions = WeightedSampler::new();

        let uniform_prior = 1.0 / (move_list.len() as f32); 

        for mv in move_list.iter() {
            let action = Action::new(mv); 
            prior.insert(action.action_id(), uniform_prior);
            value.insert(action.action_id(), 0.0);
            visit_count.insert(action.action_id(), 0);
            legal_actions.push(action); 
            unvisited_actions.push((Action::new(mv), uniform_prior))
             
        }

        let zobrist_hash = game.current_hash(); 
        let total_visit_count = 1; 
        let parent = None; 
        let rng = thread_rng(); 

        Node {
            game, prior, value, visit_count, children, parent, zobrist_hash, legal_actions, unvisited_actions, total_visit_count, rng
        }
    }

    /// Used to produce a node that is not the root node. Note: does not take in a ref to a gamestate object.
    pub fn from_game_state(mut game: GameState, parent: Option<(u64, ActionID)>, priors: &Vec<f32>) -> Result<Self, &'static str> {
        let move_list = game.get_gamestate_legal_moves(); 

        if priors.len() != move_list.len() {
            return Err("Priors tensor must be of length equal to actions.")
        }

        let mut prior = HashMap::new();
        let mut value = HashMap::new();
        let mut visit_count = HashMap::new();
        let children = HashMap::new(); 
        let mut legal_actions = Vec::with_capacity(move_list.len());
        let mut unvisited_actions = WeightedSampler::new();

        for (i, mv) in move_list.iter().enumerate() {
            let action = Action::new(mv); 
            let p = *priors.get(i).expect("tensor should be of appropriate length"); 
            prior.insert(action.action_id(), p);
            value.insert(action.action_id(), 0.0);
            visit_count.insert(action.action_id(), 0);
            legal_actions.push(action); 
            unvisited_actions.push((Action::new(mv), p)); 
             
        }

        let zobrist_hash = game.current_hash(); 
        let total_visit_count = 0; 
        let rng = thread_rng();

        Ok(Node {
            game, prior, value, visit_count, children, parent, zobrist_hash, legal_actions, unvisited_actions, total_visit_count, rng
        })
    }

    /// Returns MCTree Node from fen string of game state. 
    pub fn from_fen(fen: &str) -> Self {
        let mut game = GameState::from_fen(fen).unwrap();
        let moves_list = game.get_gamestate_legal_moves(); 

        
        let mut prior = HashMap::new();
        let mut value = HashMap::new();
        let mut visit_count = HashMap::new();
        let children = HashMap::new(); 
        let mut legal_actions = Vec::with_capacity(moves_list.len()); 
        let mut unvisited_actions = WeightedSampler::new();

        let uniform_prior = 1.0 / (moves_list.len() as f32); 

        for mv in moves_list.iter() {
            let action = Action::new(mv); 
            prior.insert(action.action_id(), uniform_prior);
            value.insert(action.action_id(), 0.0);
            visit_count.insert(action.action_id(), 0);
            legal_actions.push(action); 
            unvisited_actions.push((Action::new(mv), uniform_prior)); 

        }

        let zobrist_hash = game.current_hash(); 
        let total_visit_count = 1;
        let parent = None; 
        let rng = thread_rng();

        Node {
            game, prior, value, visit_count, children, parent, zobrist_hash, legal_actions, unvisited_actions, total_visit_count, rng
        }
    }

    /// Select child based on max UCB score of children. Only occurs when subtree fully expanded (TODO Correct for this)
    pub fn select_child(&self) -> Result<u64, &'static str> {
        if !self.fully_expanded() || self.game_over() {
            return Err("Node should be fully-expanded and non-terminal")
        }

        if let Some(best_action) = self.legal_actions()
            .iter()
            .max_by(|a, b|{
                let sqrt_total_visits = (self.total_visit_count() as f32).sqrt();
                let ucba = self.value(a.action_id()) + PUCB_C * self.prior(a.action_id()) * sqrt_total_visits / (1.0 + (self.visit_count(a.action_id()) as f32)); 
                let ucbb = self.value(b.action_id()) + PUCB_C * self.prior(b.action_id()) * sqrt_total_visits / (1.0 + (self.visit_count(b.action_id()) as f32));
                ucba.partial_cmp(&ucbb).unwrap()
            }) {
                if self.children.contains_key(&best_action.action_id()) {
                    return Ok(*self.children.get(&best_action.action_id()).unwrap()); 
                } else {
                    return Err("Child should exist")
                }
            } else {
                return Err("Node should be non-terminal.")
            }
        
    }

    /// Selects an unvisited node based on prior distribution. Only called when we have reached a node that is not
    /// fully expanded. 
    pub fn visit_unvisited(&mut self) -> Result<(GameState, Option<(u64, ActionID)>), &'static str> {
        if self.fully_expanded() {
            return Err("Node should not be fully expanded")
        }

        let action = self.unvisited_actions.sample(&mut self.rng).expect("Node not fully expanded."); 
        let parent_hash = self.game.current_hash(); 
        let mut game = self.game().clone(); 
        if game.make_move(*action.get_move()).is_err() {
            return Err("Must be a legal move"); 
        }; 
        let hash = game.current_hash(); 
        let id = action.action_id(); 
        self.children.insert(id, hash); 
        return Ok((game, Some((parent_hash, id))))
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
        let root = root_node.zhash(); 
        let mut tree = HashMap::new();
        tree.insert(root, root_node); 

        MCTS {
            tree,
            root,
        }
    }

    /// Propagates value up the MCTree. Value is the result of value network called on gamestate of leafnode. 
    pub fn propagate_values(&mut self, leaf: &mut Node, val: f32) {
        let mut action_to_path: Option<ActionID> = None; 
        let mut curr_hash = leaf.zhash();
        let mut v = val; 
        while curr_hash != self.root() {
            if let Some(curr) = self.tree.get_mut(&curr_hash) {
                curr.total_visit_count += 1;
                if let Some(action_id) = action_to_path {
                    *curr.visit_count.entry(action_id).or_default() += 1; 
                    *curr.value.entry(action_id).or_default() += (v - curr.value(action_id)) / (curr.visit_count(action_id) as f32); 
                }
                if let Some((parent_hash, parent_action)) = curr.parent() {
                    curr_hash = *parent_hash;
                    action_to_path = Some(*parent_action); 
                }
                v = -v; 
            } else {
                break; 
            }
        }
    }

    pub fn select_child(&self, node: Node) -> Result<&Node, &'static str> {
        if !node.fully_expanded() || node.game_over() {
            return Err("Node must be fully expanded and non-terminal.")
        }
        let hash = node.select_child().expect("Node is fully expanded and non-term.");
        Ok(self.tree.get(&hash).expect("Node should exist"))
    }

    pub fn visit_unvisited(&mut self, node: Node, priors: &Vec<f32>) {

    }

}