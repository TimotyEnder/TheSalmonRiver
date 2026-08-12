use godot::classes::{INode, Node};
use godot::global::Key;
use godot::prelude::*;

use crate::game_manager::game::Game;
use crate::game_manager::player_control_scheme::PlayerControlScheme;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager {
    base: Base<Node>,
    current_game: Option<Game>,
    control_schemes: Vec<Gd<PlayerControlScheme>>,
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            current_game: None,
            control_schemes: Vec::new(),
        }
    }

    fn ready(&mut self) {
        godot_print!("GameManager ready!");
    }
}

#[godot_api]
impl GameManager {
    #[func]
    pub fn create_game(&mut self, num_players: u8, rounds_to_win: u32, round_time: u32) {
        self.current_game = Some(Game::new(
            num_players,
            rounds_to_win as usize,
            round_time as usize,
        ));
    }
    #[func]
    pub fn player_won_round(&mut self, player_num: u8) {
        if let Some(ref mut game) = self.current_game {
            game.log_player_win(player_num);
        }
    }
    #[func]
    pub fn round_tie(&mut self) {
        if let Some(ref mut game) = self.current_game {
            game.log_tie();
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
    #[func]
    pub fn get_player_score(&self, player_num: u8) -> u32 {
        if let Some(ref game) = self.current_game {
            return game.get_score_for_player(player_num);
        } else {
            return 0;
        }
    }
    #[func]
    pub fn get_round_timer_secs(&self) -> u32 {
        if let Some(ref game) = self.current_game {
            return game.get_round_time() as u32;
        }
        return 0;
    }
    #[func]
    pub fn request_player_controls(&mut self, player_num: u8) -> Gd<PlayerControlScheme> {
        while self.control_schemes.len() < player_num as usize {
            self.control_schemes
                .push(Gd::from_object(PlayerControlScheme {
                    jump_key: Key::NONE,
                    left_key: Key::NONE,
                    right_key: Key::NONE,
                    duck_key: Key::NONE,
                    punch_use_key: Key::NONE,
                    grab_throw_key: Key::NONE,
                }));
        }
        return self.control_schemes[player_num as usize - 1].clone();
    }
    #[func]
    pub fn save_player_controls(&mut self, player_num: u8, control: Gd<PlayerControlScheme>) {
        while self.control_schemes.len() < player_num as usize {
            self.control_schemes
                .push(Gd::from_object(PlayerControlScheme {
                    jump_key: Key::NONE,
                    left_key: Key::NONE,
                    right_key: Key::NONE,
                    duck_key: Key::NONE,
                    punch_use_key: Key::NONE,
                    grab_throw_key: Key::NONE,
                }));
        }
        self.control_schemes[player_num as usize - 1] = control;
    }
}
