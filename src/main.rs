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
        
        game_state.draw_board();
        
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}