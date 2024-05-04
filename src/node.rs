use macroquad::prelude::*;

#[derive(Clone)]
pub struct Node {
    pub position: Vec2
}

impl Node {
    pub fn new(x: f32, y:f32) -> Self {
        Node {
            position: Vec2{x, y}
        }
    }
}