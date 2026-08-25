use godot::classes::{INode, Node};
use godot::global::Key;
use godot::prelude::*;

use crate::game_managers::audio_manager::AudioManager;
use crate::game_managers::game::Game;
use crate::game_managers::player_control_scheme::PlayerControlScheme;
use crate::game_managers::save_manager::SaveManager;
use crate::game_scripts::game_utils::PlayerCharacterType::{self, Bear};
use crate::settings_state::SettingsState;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameManager {
    base: Base<Node>,
    current_game: Option<Game>,
    control_schemes: Vec<Gd<PlayerControlScheme>>,
    settings_state: Gd<SettingsState>,
    save_manager: Gd<SaveManager>,
    character_types: Vec<PlayerCharacterType>,
}

#[godot_api]
impl INode for GameManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            current_game: None,
            control_schemes: Vec::new(),
            save_manager: Gd::from_object(SaveManager {}),
            settings_state: Gd::from_object(SettingsState::default()),
            character_types: vec![Bear; 4],
        }
    }

    fn ready(&mut self) {
        godot_print!("GameManager ready!");
        self.control_schemes = self
            .save_manager
            .bind_mut()
            .load_control_schemes_from_file();
        self.settings_state = self.save_manager.bind().load_settings_state();
        self.base_mut().call_deferred("apply_audio_settings", &[]);
    }
}

#[godot_api]
impl GameManager {
    pub fn apply_audio_settings(&mut self) {
        if let Some(mut audio) = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal")
        {
            audio
                .bind_mut()
                .set_game_volume(self.settings_state.bind().game_volume_percent);
        }
    }

    pub fn create_game(&mut self, num_players: u8, rounds_to_win: u32, round_time: u32) {
        self.current_game = Some(Game::new(
            num_players,
            rounds_to_win as usize,
            round_time as usize,
        ));
        self.save_manager
            .bind_mut()
            .save_control_schemes(self.control_schemes.clone());
    }

    pub fn player_won_round(&mut self, player_num: u8) {
        if let Some(ref mut game) = self.current_game {
            game.log_player_win(player_num);
        }
    }

    pub fn round_tie(&mut self) {
        if let Some(ref mut game) = self.current_game {
            game.log_tie();
        }
    }

    pub fn get_current_round_number_one_based(&self) -> u32 {
        if let Some(ref game) = self.current_game {
            return game.get_current_round() + 1 as u32;
        } else {
            return 0;
        }
    }

    pub fn get_player_count(&mut self) -> u32 {
        if let Some(ref game) = self.current_game {
            return game.get_player_number() as u32;
        } else {
            return 0;
        }
    }

    pub fn get_player_score(&self, player_num: u8) -> u32 {
        if let Some(ref game) = self.current_game {
            return game.get_score_for_player(player_num);
        } else {
            return 0;
        }
    }

    pub fn get_round_timer_secs(&self) -> u32 {
        if let Some(ref game) = self.current_game {
            return game.get_round_time() as u32;
        }
        return 0;
    }

    pub fn get_rounds_to_win(&self) -> u32 {
        if let Some(ref game) = self.current_game {
            return game.get_rounds_to_win() as u32;
        }
        return 0;
    }

    pub fn request_settings_state(&mut self) -> Gd<SettingsState> {
        self.settings_state.clone()
    }

    pub fn save_settings_state(&mut self, state: Gd<SettingsState>) {
        self.settings_state = state;
        self.save_manager
            .bind_mut()
            .save_settings_state(self.settings_state.clone());
    }

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

    pub fn is_game_finished(&self) -> bool {
        if let Some(ref game) = self.current_game {
            return game.get_winner().is_some();
        }
        return false;
    }

    pub fn get_winner(&self) -> u8 {
        if let Some(ref game) = self.current_game {
            return game.get_winner().unwrap_or(0);
        } else {
            return 0;
        }
    }
    pub fn set_player_character_type(&mut self, player_num: u8, char_type: PlayerCharacterType) {
        self.character_types[player_num as usize] = char_type;
    }
    pub fn get_player_character_type(&self, player_num: u8) -> PlayerCharacterType {
        self.character_types[player_num as usize]
    }
}
