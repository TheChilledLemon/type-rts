mod gamestate;
mod node;
mod input;
mod unit;

use gamestate::GameState;
use input::get_input;
use macroquad::prelude::*;

#[macroquad::main("Type RTS")]
async fn main() {
    let mut game_state: GameState = GameState::new(5, 5);
    loop {        
        clear_background(GRAY);

        if let Some(command) = get_input(&mut game_state) {
            println!("{:?}", command);
        }

        
        game_state.update_nodes();
        
        game_state.draw_gamestate();

        next_frame().await
    }
}