use crate::gamestate::{Command, GameState};
use macroquad::{prelude::*, ui::{root_ui, widgets}};

pub fn get_input(game_state: &mut GameState) -> Option<Command> {
    let player_buf = &mut game_state.player_buffer;

    // Always set input focused to the input.
    root_ui().set_input_focus(0);

    // Add text input widget to root ui.
    widgets::InputText::new(0)
        .position(vec2(0.0, screen_height() - 30.0))
        .ui(&mut root_ui(), &mut *player_buf);

    // If the enter key's been pressed.
    if let Some(key) = get_last_key_pressed() {
        if key.eq(&KeyCode::Enter) {
            let mut iter = player_buf.splitn(3, ' ');
            let source = String::from(iter.next().unwrap_or(""));
            let destination = String::from(iter.next().unwrap_or(""));
            let quantity_str = String::from(iter.next().unwrap_or(""));

            // Check if there's anything left in the buffer.
            if iter.next().is_some() {
                return None
            }
            player_buf.clear();

            // If we have an invalid set of arguments.
            if source.is_empty() || destination.is_empty() || quantity_str.is_empty() {
                return None
            }
            
            // Parse quantity as usize then return Command if successful.
            if let Some(quantity) = quantity_str.parse::<usize>().ok() {
                return Some(Command{
                    source,
                    destination,
                    quantity,
                })
            }            
        }
    }
    None
}