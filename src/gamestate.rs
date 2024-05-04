use macroquad::prelude::*;
use crate::node;

#[derive(Debug)]
pub struct Command {
    pub source: String,
    pub destination: String,
    pub quantity: usize
}

pub struct GameState {
    pub player_buffer: String,
    pub board: Vec<Vec<node::Node>>
}

impl GameState {
    pub fn new(rows: usize, cols: usize) -> Self {
        if rows == 0 && cols == 0 {
            panic!("Invalid board dimensions");
        }
        GameState {
            player_buffer: String::new(),
            board: vec![vec![node::Node{}; cols]; rows]
        }
    }

    pub fn draw_board(&mut self) {
        for i in 1..5 {
            draw_line(40.0, i as f32 * 40.0, 100.0, 200.0, 15.0, BLUE);
        }
    }
}