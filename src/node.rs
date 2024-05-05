use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Player {
    None,
    Player,
    AI
}

impl Player {
    pub fn value(&self) -> PlayerStruct {
        match *self {
            Player::None => PlayerStruct {color: WHITE, respawn_rate: 3.0},
            Player::Player => PlayerStruct {color: BLUE, respawn_rate: 1.0},
            Player::AI => PlayerStruct {color: RED, respawn_rate: 2.5},
        }
    }
}

pub struct PlayerStruct {
    pub color: Color,
    pub respawn_rate: f64,
}

#[derive(PartialEq, Debug)]
pub struct Node {
    pub position: Vec2,
    pub name: String,
    pub row_index: usize,
    pub col_index: usize,
    pub owner: Player,
    pub num_units: u32,
    pub last_spawn: f64
}

impl Node {
    pub fn new(x: f32, y:f32, row_index: usize, col_index: usize, start_time: f64) -> Self {
        let letter = char::from_u32(row_index as u32 + 65).unwrap();
        let num = char::from_digit(col_index as u32, 10).unwrap();
        let mut name = String::new();
        name.push(letter);
        name.push(num);
        Node {
            position: Vec2{x, y},
            row_index,
            col_index,
            name,
            owner: Player::None,
            num_units: 0,
            last_spawn: start_time
        }
    }
}