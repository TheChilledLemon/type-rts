use crate::node::Player;
use macroquad::prelude::*;

pub struct Unit {
    pub position: Vec2,
    pub owner: Player,
    pub speed: f32,
    pub radius: f32,
    pub source: Vec2,
    pub destination: Vec2
}

impl Unit {
    pub fn new(x: f32, y:f32, owner: Player, source: Vec2, destination: Vec2) -> Self {
        Unit {
            position: Vec2{x, y},
            speed: 1.0,
            radius: 5.0,
            owner,
            source,
            destination,
        }
    }
}

