mod gamestate;
mod node;
mod input;
mod unit;
mod ai;

use ai::AI;
use gamestate::{GameState, Command};
use input::get_input;
use macroquad::prelude::*;

#[macroquad::main("Type RTS")]
async fn main() {
    let mut game_state: GameState = GameState::new(5, 5);
    let mut ai: AI = AI::new(get_time());
    loop {        
        clear_background(GRAY);

        // Capture Input
        if let Some(command) = get_input(&mut game_state) {
            let src_tup = Command::to_index(&command.source).unwrap();
            let dest_tup = Command::to_index(&command.destination).unwrap();
            game_state.add_unit(src_tup, dest_tup, command.quantity);
        }

        if let Some(ai_move) = ai.get_move(&game_state) {
            game_state.add_unit(ai_move.source, ai_move.destination, ai_move.quantity);
        }

        // Process Logic
        game_state.update_nodes();
        game_state.update_units();
        
        // Draw Game
        game_state.draw_gamestate();

        next_frame().await
    }
}