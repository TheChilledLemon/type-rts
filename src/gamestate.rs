use macroquad::{math, prelude::*};
use crate::node::{Node};

#[derive(Debug)]
pub struct Command {
    pub source: String,
    pub destination: String,
    pub quantity: usize
}

pub struct GameState {
    pub player_buffer: String,
    pub board: Vec<Vec<Node>>,
    rows: usize,
    cols: usize
}

impl GameState {
    pub fn new(rows: usize, cols: usize) -> Self {
        if rows == 0 && cols == 0 {
            panic!("Invalid board dimensions");
        }

        GameState {
            player_buffer: String::new(),
            board: vec![vec![Node::new(0.0, 0.0); cols]; rows],
            rows,
            cols
        }
    }

    pub fn draw_board(&mut self) {
        self.update_node_positions();
        for row in self.board.iter() {
            for node in row {
                let radius = screen_width().min(screen_height()) / 20.0;
                draw_circle(node.position.x, node.position.y, radius, WHITE);
            }
        }
    }

    fn update_node_positions(&mut self) {
        let offset = 100.0;
        let x_increment = get_increment(offset, screen_width() - offset, self.cols as f32);
        let y_increment = get_increment(offset, screen_height() - offset, self.rows as f32);
        let mut board: Vec<Vec<Node>> = Vec::new();
        
        for y_num in 0..self.rows {
            let y = offset + (y_increment * y_num as f32);
            let mut row: Vec<Node> = Vec::new();
            for x_num in 0..self.cols {
                let x = offset + (x_increment * x_num as f32);
                row.push(Node::new(x, y));
            }
            board.push(row);
        }
        self.board = board;
    }
}

fn get_increment(edge_a: f32, edge_b: f32, num_items: f32) -> f32 {
    (edge_b - edge_a).abs() / (num_items - 1 as f32)
}