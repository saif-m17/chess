use crate::bitboards::Bitboard;
use crate::moves::{Direction, Square};  
use crate::attacktables::RAYS; 
use once_cell::sync::Lazy;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;


const BISHOP_INDEX_BITS: usize = 9;
const ROOK_INDEX_BITS: usize = 12;

const ROOK_DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];
const BISHOP_DIRECTIONS: [Direction; 4] = [Direction::NorthEast, Direction::NorthWest, Direction::SouthEast, Direction::SouthWest]; 

#[derive(Clone)]
struct Magic<const N: usize> {
    magic_num: u64,
    direction_mask: Bitboard, 
    attack_table: Vec<Option<Bitboard>>, 
}

impl<const N: usize> Magic<N> {
    pub fn new_magic(magic_num: u64, direction_mask: Bitboard, attack_table: Vec<Option<Bitboard>>) -> Magic<N> {
        Magic {
            magic_num,
            direction_mask,
            attack_table,
        }
    }
}

impl<const N: usize> Default for Magic<N> {
    fn default() -> Self {
        Magic {
            magic_num: 0,
            direction_mask: 0,
            attack_table: vec![None; N],
        }
    }
}

type BishopMagic = Magic<{ 1 << BISHOP_INDEX_BITS }>;
type RookMagic   = Magic<{ 1 << ROOK_INDEX_BITS }>;

fn get_rook_magics() -> Vec<RookMagic> {
    let directions_list = ROOK_DIRECTIONS;
    let mut magics: Vec<RookMagic> = vec![RookMagic::default(); 64]; 
    for square in 0u8..64 {
        let directions = get_directions_bb(square, directions_list);
        let rays = directions[0] | directions[1] | directions[2] | directions[3]; 
        loop {
            let candidate_magic = generate_magic(); 
            if let Ok(attack_table) = check_table(candidate_magic, rays, directions_list, square, ROOK_INDEX_BITS) {
                magics[square as usize] = RookMagic::new_magic(candidate_magic, rays, attack_table); 
                break 
            }
        } 
    }
    magics
}

fn get_bishop_magics() -> Vec<BishopMagic> {
    let directions_list = BISHOP_DIRECTIONS;
    let mut magics: Vec<BishopMagic> = vec![BishopMagic::default(); 64];
    for square in 0u8..64 {
        let directions = get_directions_bb(square, directions_list);
        let rays = directions[0] | directions[1] | directions[2] | directions[3];
        loop {
            let candidate_magic = generate_magic();
            if let Ok(attack_table) = check_table(candidate_magic, rays, directions_list, square, BISHOP_INDEX_BITS) {
                magics[square as usize] = BishopMagic::new_magic(candidate_magic, rays, attack_table); 
                break
            }
        }
    }

    magics
}

fn check_table(candidate_magic: u64, rays: Bitboard, directions_list: [Direction; 4], square: u8, index_bits: usize) -> Result<Vec<Option<Bitboard>>, MagicError>   {
    let mut attack_table: Vec<Option<Bitboard>> =  vec![None; 1 << index_bits];
    let mut blockers = 0u64;

    loop {
        let magic_index = blockers.wrapping_mul(candidate_magic) >> (64 - index_bits);
        if attack_table[magic_index as usize].is_some() {
            return Err(MagicError::CollisionDetected(magic_index as usize))
        } else {
            let attacks = get_attacks(directions_list, blockers, square); 
            attack_table[magic_index as usize] = Some(attacks); 
        }
        blockers = blockers.wrapping_sub(rays) & rays; 
        if blockers == 0u64 {
            break; 
        }
        
    }
    Ok(attack_table)

}


fn get_directions_bb(square: u8, directions_list: [Direction; 4]) -> [Bitboard; 4] {
    [RAYS[square as usize][directions_list[0] as usize],
    RAYS[square as usize][directions_list[1] as usize],
    RAYS[square as usize][directions_list[2] as usize],
    RAYS[square as usize][directions_list[3] as usize],]
}

fn generate_magic() -> u64 {
    let mut rng = StdRng::seed_from_u64(42);
    let candidate_magic = rng.r#gen::<u64>() & rng.r#gen::<u64>() & rng.r#gen::<u64>();
    candidate_magic
}

fn get_attacks(directions: [Direction; 4], blockers: Bitboard, square: u8) -> Bitboard {
    let mut attacks = 0u64; 
    for direction in directions {
        let direction_ray = RAYS[square as usize][direction as usize];
        let relevant_blockers = direction_ray & blockers; 
        let first_blocker = match direction {
            Direction::North => relevant_blockers.trailing_zeros(),
            Direction::NorthEast => relevant_blockers.trailing_zeros(),
            Direction::NorthWest => relevant_blockers.trailing_zeros(),
            Direction::West => relevant_blockers.trailing_zeros(),
            Direction::East => 64 - relevant_blockers.leading_zeros(),
            Direction::South => 64 - relevant_blockers.leading_zeros(),
            Direction::SouthEast => 64 - relevant_blockers.leading_zeros(),
            Direction::SouthWest => 64 - relevant_blockers.leading_zeros(),
        }; 
        let first_blocker_ray = RAYS[first_blocker as usize][direction as usize];
        let attack_this_direction = first_blocker_ray ^ direction_ray; 
        attacks |= attack_this_direction; 
    }
    attacks
}

pub enum MagicError {
    CollisionDetected(usize),
}


static ROOK_MAGICS: Lazy<Vec<RookMagic>> = Lazy::new(|| get_rook_magics());
static BISHOP_MAGICS: Lazy<Vec<BishopMagic>> = Lazy::new(||get_bishop_magics()); 



