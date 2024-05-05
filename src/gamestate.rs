use macroquad::prelude::*;
use crate::node::{Node, Player};
use crate::unit::Unit;

pub struct Command {
    pub source: String,
    pub destination: String,
    pub quantity: u32,
    pub owner: Player
}

impl Command {
    pub fn new(source: String, destination: String, quantity: u32, owner: Player, gamestate: &GameState) -> Option<Self> {
        match Command::to_index(&source) {
            Some(src_tup) => {
                match Command::to_index(&destination) {
                    Some(dest_tup) => {
                        if gamestate.valid_move(src_tup, dest_tup, quantity) {
                            Some(Command {
                                source,
                                destination,
                                quantity,
                                owner
                            })
                        } else {
                            None
                        }
                    }
                    _ => None
                }
            }
            _ => None
        }
    }

    pub fn to_index(coord: &String) -> Option<(usize, usize)> {
        let chars: Vec<char> = coord.chars().collect();
        if chars.len() == 2 {
            let row: usize = chars[0] as usize - 65;
            let col: usize = chars[1].to_digit(10)? as usize;
            Some((row, col))
        } else {
            None
        }
    }
}

pub struct GameState {
    pub player_buffer: String,
    pub board: Vec<Vec<Node>>,
    pub units: Vec<Unit>,
    rows: usize,
    cols: usize
}

impl GameState {
    pub fn new(rows: usize, cols: usize) -> Self {
        if rows == 0 && cols == 0 {
            panic!("Invalid board dimensions");
        }

        let start_time = get_time();
        let mut board: Vec<Vec<Node>> = Vec::new(); 

        for y_num in 0..rows {
            let mut row: Vec<Node> = Vec::new();
            for x_num in 0..cols {
                row.push(Node::new(0.0, 0.0, y_num, x_num, start_time))
            }
            board.push(row);
        }

        board[0][0].owner = Player::Player;
        board[rows - 1][cols - 1].owner = Player::AI;

        GameState {
            player_buffer: String::new(),
            board,
            units: Vec::new(),
            rows,
            cols
        }
    }

    pub fn add_unit(&mut self, source: (usize, usize), dest: (usize, usize), quantity: u32) {
        let src_node = &mut self.board[source.0][source.1];
        src_node.num_units -= quantity;
        self.units.push(Unit::new(src_node.position.x, src_node.position.y,
            src_node.owner.clone(), quantity, source, dest))
    }

    pub fn valid_move(& self, source: (usize, usize), dest: (usize, usize), quantity: u32) -> bool {
        if !self.inbounds(source) || !self.inbounds(dest) {
            return false;
        }
        let src_node = &self.board[source.0][source.1];
        let dest_node = &self.board[dest.0][dest.1];
        if !matches!(src_node.owner, Player::Player) {
            return false;
        }
        if src_node.num_units < quantity {
            return false;
        }
        if !self.get_nearest_neighbors(src_node).contains(&dest_node) {
            return false;
        }
        true
    }

    pub fn draw_gamestate(&mut self) {
        self.update_node_positions();
        let radius = screen_width().min(screen_height()) / 20.0;
        let thickness = radius / 5.0;
        self.draw_lines(thickness);
        self.draw_units(radius / 3.0);
        self.draw_nodes(radius);
    }

    pub fn update_units(&mut self) {
        self.units.retain_mut(|unit: &mut Unit| unit.update(&mut self.board));
    }

    pub fn update_nodes(&mut self) {
        let time_now = get_time();
        for row in self.board.iter_mut() {
            for node in row {
                let rate = node.owner.value().respawn_rate;
                if time_now - node.last_spawn >= rate {
                    node.num_units += 1;
                    node.last_spawn = time_now;
                }
            }
        }
    }

    fn draw_lines(&mut self, thickness: f32) {
        for row in self.board.iter() {
            for node in row {
                for i in self.get_nearest_neighbors(node) {
                    draw_line(node.position.x, node.position.y,
                        i.position.x, i.position.y, thickness, LIGHTGRAY)
                }
            }
        }
    }

    fn draw_units(&mut self, radius: f32) {
        for unit in self.units.iter_mut() {
            unit.draw(radius);
        }
    }

    fn draw_nodes(&mut self, radius: f32) {
        let font_size = (radius / 2.0) as u16;
        for row in self.board.iter() {
            for node in row {
                draw_circle(node.position.x, node.position.y, radius, node.owner.value().color);

                let label_center = get_text_center(&node.name, None, font_size, 1.0, 0.0);
                draw_text_ex(&node.name, node.position.x - label_center.x, node.position.y - label_center.y - 10.0,
                    TextParams{
                        font_size,
                        color: BLACK,
                        ..Default::default()
                    });
                
                let num_str = node.num_units.to_string();
                let num_center = get_text_center(&num_str, None, font_size, 1.0, 0.0);
                draw_text_ex(&num_str, node.position.x - num_center.x, node.position.y - num_center.y + 10.0,
                    TextParams{
                        font_size,
                        color: BLACK,
                        ..Default::default()
                    });
            }
        }
    }

    fn update_node_positions(&mut self) {
        let offset = 100.0;
        let x_increment = get_increment(offset, screen_width() - offset, self.cols);
        let y_increment = get_increment(offset, screen_height() - offset, self.rows);
        
        for y_num in 0..self.rows {
            let y = offset + (y_increment * y_num as f32);
            for x_num in 0..self.cols {
                let x = offset + (x_increment * x_num as f32);
                self.board[y_num][x_num].position = Vec2{x, y};
            }
        }
    }

    pub fn get_nearest_neighbors(&self, node: &Node) -> Vec<&Node> {
        let mut res: Vec<&Node> = Vec::new();
        if node.row_index > 0 {
            res.push(&self.board[node.row_index - 1][node.col_index])
        }
        if node.col_index > 0 {
            res.push(&self.board[node.row_index][node.col_index - 1])
        }
        if node.row_index < self.board.len() - 1 {
            res.push(&self.board[node.row_index + 1][node.col_index])
        }
        if node.col_index < self.board[0].len() - 1 {
            res.push(&self.board[node.row_index][node.col_index + 1])
        }
        res
    }

    fn inbounds(&self, coord: (usize, usize)) -> bool {
        if coord.0 >= self.rows || coord.1 >= self.cols {
            false
        } else {
            true
        }
    }
}

fn get_increment(edge_a: f32, edge_b: f32, num_items: usize) -> f32 {
    (edge_b - edge_a).abs() / (num_items - 1) as f32
}