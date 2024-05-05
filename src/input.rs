use crate::gamestate::{Command, GameState};
use crate::node::Player;
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
    match get_last_key_pressed() {
        None => None,
        Some(key) => {
            match key {
                KeyCode::Enter => {
                    let input_split: Vec<_> = player_buf.splitn(3, ' ').map(|a| {
                        String::from(a)
                    }).collect();
                    player_buf.clear();

                    match TryInto::<[String; 3]>::try_into(input_split) {
                        Ok(array_input) => {
                            let [source, destination, quantity_str] = array_input;

                            match quantity_str.parse::<u32>() {
                                Ok(quantity) => Command::new(source, destination, quantity, Player::Player, &game_state),
                                _ => None
                            }
                        }
                        _ => None,
                    }
                },
                _ => None
            }
        }
    }
}