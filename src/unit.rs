use crate::{gamestate::GameState, node::{Node, Player}};
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Unit {
    pub position: Vec2,
    pub owner: Player,
    pub radius: f32,
    pub quantity: u32,
    pub source: (usize, usize),
    pub destination: (usize, usize)
}

impl Unit {
    pub fn new(x: f32, y:f32, owner: Player, quantity: u32, source: (usize, usize), destination: (usize, usize)) -> Self {
        Unit {
            position: Vec2{x, y},
            quantity,
            owner,
            radius: 0.0,
            source,
            destination,
        }
    }

    pub fn update(&mut self, board: &mut Vec<Vec<Node>>) -> bool {
        // Update position
        let dest_node = &mut board[self.destination.0][self.destination.1];
        self.position += (dest_node.position - self.position).normalize() * (screen_width().min(screen_width()) / 175.0) / self.quantity as f32;

        // Check if we reach destination
        if self.position.distance(dest_node.position) < 5.0 {
            if self.owner == dest_node.owner {
                dest_node.num_units += self.quantity;
            } else {
                if self.quantity >= dest_node.num_units {
                    dest_node.num_units = self.quantity - dest_node.num_units;
                    dest_node.owner = self.owner.clone();
                } else {
                    dest_node.num_units -= self.quantity;
                }
            }
            return false;
        }
        true
    }

    pub fn draw(&mut self, radius: f32) {
        self.radius = radius;
        draw_circle(self.position.x, self.position.y, radius,self.owner.value().color);

        let font_size = radius as u16;
        let num_str = self.quantity.to_string();
        let num_center = get_text_center(&num_str, None, font_size, 1.0, 0.0);
        draw_text_ex(&num_str, self.position.x - num_center.x, self.position.y - num_center.y,
            TextParams{
                font_size,
                color: BLACK,
                ..Default::default()
            });
    }
}

