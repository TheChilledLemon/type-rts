use macroquad::time::get_time;

use crate::{gamestate::GameState, node::{Node, Player}};

pub struct AI {
    pub command_rate: f64,
    pub last_command: f64
}

pub struct Move {
    pub source: (usize, usize),
    pub destination: (usize, usize),
    pub quantity: u32
}

impl AI {
   pub fn new(last_command: f64) -> Self {
    AI {
        command_rate: 3.0,
        last_command
    }
   }

   pub fn get_move(&mut self, gamestate: &GameState) -> Option<Move> {
    let time_now = get_time();
    if time_now - self.last_command > 3.0 {
        self.last_command = time_now;
        let reachable_nodes: Vec<&Node> = get_reachable(gamestate);
        let dest_node = reachable_nodes.iter().max_by_key(|node| scoring_func(gamestate, node)).unwrap();

        let neighbors = gamestate.get_nearest_neighbors(&dest_node);
        let max_neigh = neighbors.iter().filter(|neigh| neigh.owner == Player::AI).max_by_key(|neigh| neigh.num_units).unwrap();
        if max_neigh.num_units as i32 - dest_node.num_units as i32 > 5 {
            return Some(Move {
                source: (max_neigh.row_index, max_neigh.col_index),
                destination: (dest_node.row_index, dest_node.col_index),
                quantity: dest_node.num_units + 5
            });
        }
    }
    None
    }

}

fn get_reachable(gamestate: &GameState) -> Vec<&Node> {
    gamestate.board.iter().flatten().filter(|node| {
        let neighbors = gamestate.get_nearest_neighbors(node);
        if neighbors.iter().any(|neigh| neigh.owner == Player::AI) {
            true
        } else {
            false
        }
    }).collect()
}

fn scoring_func(gamestate: &GameState, node: &Node) -> i32 {
    let weight: i32;
    match node.owner {
        Player::AI => { weight = 1; }
        Player::None => { weight = 2; }
        Player::Player => { weight = 3; }
    }

    let neighbors = gamestate.get_nearest_neighbors(node);
    let max_neigh = neighbors.iter().filter(|neigh| neigh.owner == Player::AI)
    .map(|neigh| neigh.num_units).max().unwrap();
    
    let disparity = max_neigh as i32 - node.num_units as i32;
    disparity * weight
}
    