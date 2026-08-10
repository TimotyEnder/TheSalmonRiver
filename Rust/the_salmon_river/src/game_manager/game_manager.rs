use godot::classes::{INode, Node};
use godot::prelude::*;

use crate::game_manager::game::Game;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager {
    base: Base<Node>,
    current_game: Option<Game>,
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            current_game: None,
        }
    }

    fn ready(&mut self) {
        godot_print!("GameManager ready!");
    }
}

#[godot_api]
impl GameManager {
    #[func]
    pub fn create_game(&mut self, num_players: u8, rounds_to_win: u32) {
        self.current_game = Some(Game::new(num_players, rounds_to_win as usize));
    }
    #[func]
    pub fn player_won_round(&mut self, player_num: u8) {
        if let Some(ref mut game) = self.current_game {
            game.log_player_win(player_num);
        }
    }
    #[func]
    pub fn get_current_round_number_one_based(&self) -> u32 {
        if let Some(ref game) = self.current_game {
            return game.get_current_round() + 1 as u32;
        } else {
            return 0;
        }
    }
    #[func]
    pub fn get_player_count(&mut self) -> u32 {
        if let Some(ref game) = self.current_game {
            return game.get_player_number() as u32;
        } else {
            return 0;
        }
    }
}
