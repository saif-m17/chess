use std::collections::HashMap;
use crate::GameState;
use crate::action_space::{Action, ActionID}; 
use crate::Move; 

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
    unvisited_actions: Vec<Action>,
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
    pub fn parent(&self) -> &Option<(u64, ActionID)> {&self.parent}
    pub fn unvisited_actions(&self) -> &Vec<Action> {&self.unvisited_actions}

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
        let mut unvisited_actions = Vec::with_capacity(move_list.len());

        let uniform_prior = 1.0 / (move_list.len() as f32); 

        for mv in move_list.iter() {
            let action = Action::new(mv); 
            prior.insert(action.action_id(), uniform_prior);
            value.insert(action.action_id(), 0.0);
            visit_count.insert(action.action_id(), 0);
            legal_actions.push(action); 
            unvisited_actions.push(Action::new(mv))
             
        }

        let zobrist_hash = game.current_hash(); 
        let total_visit_count = 1; 
        let parent = None; 

        Node {
            game, prior, value, visit_count, children, parent, zobrist_hash, legal_actions, unvisited_actions, total_visit_count,
        }
    }

    /// Used to produce a node that is not the root node 
    pub fn from_game_state(mut game: GameState, parent: Option<(u64, ActionID)>) -> Self {
        let move_list = game.get_gamestate_legal_moves(); 

        let mut prior = HashMap::new();
        let mut value = HashMap::new();
        let mut visit_count = HashMap::new();
        let children = HashMap::new(); 
        let mut legal_actions = Vec::with_capacity(move_list.len());
        let mut unvisited_actions = Vec::with_capacity(move_list.len());

        let uniform_prior = 1.0 / (move_list.len() as f32); 

        for mv in move_list.iter() {
            let action = Action::new(mv); 
            prior.insert(action.action_id(), uniform_prior);
            value.insert(action.action_id(), 0.0);
            visit_count.insert(action.action_id(), 0);
            legal_actions.push(action); 
            unvisited_actions.push(Action::new(mv)); 
             
        }

        let zobrist_hash = game.current_hash(); 
        let total_visit_count = 0; 

        Node {
            game, prior, value, visit_count, children, parent, zobrist_hash, legal_actions, unvisited_actions, total_visit_count,
        }
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
        let mut unvisited_actions = Vec::with_capacity(moves_list.len());

        let uniform_prior = 1.0 / (moves_list.len() as f32); 

        for mv in moves_list.iter() {
            let action = Action::new(mv); 
            prior.insert(action.action_id(), uniform_prior);
            value.insert(action.action_id(), 0.0);
            visit_count.insert(action.action_id(), 0);
            legal_actions.push(action); 
            unvisited_actions.push(Action::new(mv)); 

        }

        let zobrist_hash = game.current_hash(); 
        let total_visit_count = 1;
        let parent = None; 

        Node {
            game, prior, value, visit_count, children, parent, zobrist_hash, legal_actions, unvisited_actions, total_visit_count, 
        }
    }

    /// Select child based on max UCB score of children. Only occurs when subtree fully expanded (TODO Correct for this)
    pub fn select_child(&mut self) -> (Option<u64>, Option<Node>) {
        if let Some(best_action) = self.legal_actions()
            .iter()
            .max_by(|a, b|{
                let sqrt_total_visits = (self.total_visit_count() as f32).sqrt();
                let ucba = self.value(a.action_id()) + PUCB_C * self.prior(a.action_id()) * sqrt_total_visits / (1.0 + (self.visit_count(a.action_id()) as f32)); 
                let ucbb = self.value(b.action_id()) + PUCB_C * self.prior(b.action_id()) * sqrt_total_visits / (1.0 + (self.visit_count(b.action_id()) as f32));
                ucba.partial_cmp(&ucbb).unwrap()
            }) {

                if self.children.contains_key(&best_action.action_id()) {
                    return (Some(*self.children.get(&best_action.action_id()).unwrap()), None); 
                } else {
                    let mut game = self.game().clone(); 
                    if game.make_move(*best_action.get_move()).is_err() {
                        return (None, None)
                    }; 
                    let hash = game.current_hash(); 
                    let id = best_action.action_id(); 
                    self.children.insert(id, hash); 
                    let node = Node::from_game_state(game, Some((self.zhash(), id))); 
                    return (Some(hash), Some(node))
                }
            } else {
                return (None, None)
            }
        
    }

    /// Selects an unvisited node based on prior distribution. Only called when we have reached a node that is not
    /// fully expanded. 
    pub fn visit_unvisited(&self) -> Node {
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

}