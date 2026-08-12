use godot::{
    classes::{Button, IPanel, Panel, RichTextLabel},
    global::Key,
    prelude::*,
};

use crate::{
    game_manager::{game_manager::GameManager, player_control_scheme::PlayerControlScheme},
    ui::keymapping_button::KeyMappingButton,
};

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct PlayerControlsPanel {
    base: Base<Panel>,
    key_mapping_button_jump: Option<Gd<KeyMappingButton>>,
    key_mapping_button_left: Option<Gd<KeyMappingButton>>,
    key_mapping_button_right: Option<Gd<KeyMappingButton>>,
    key_mapping_button_duck: Option<Gd<KeyMappingButton>>,
    key_mapping_button_punch_use: Option<Gd<KeyMappingButton>>,
    key_mapping_button_grab_throw: Option<Gd<KeyMappingButton>>,
    player_label: Option<Gd<RichTextLabel>>,
    player_num_assigned: Option<u8>,
}

#[godot_api]
impl IPanel for PlayerControlsPanel {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            key_mapping_button_duck: None,
            key_mapping_button_grab_throw: None,
            key_mapping_button_left: None,
            key_mapping_button_jump: None,
            key_mapping_button_punch_use: None,
            key_mapping_button_right: None,
            player_label: None,
            player_num_assigned: None,
        }
    }
    fn ready(&mut self) {
        self.key_mapping_button_jump = self.base().find_child("BindKeyMapping").and_then(|bkp| {
            bkp.find_child("KeyMappingButtonJump")
                .and_then(|key| key.try_cast::<KeyMappingButton>().ok())
        });
        self.key_mapping_button_duck = self.base().find_child("BindKeyMapping").and_then(|bkp| {
            bkp.find_child("KeyMappingButtonDuck")
                .and_then(|key| key.try_cast::<KeyMappingButton>().ok())
        });
        self.key_mapping_button_grab_throw =
            self.base().find_child("BindKeyMapping").and_then(|bkp| {
                bkp.find_child("KeyMappingButtonGrabThrow")
                    .and_then(|key| key.try_cast::<KeyMappingButton>().ok())
            });
        self.key_mapping_button_left = self.base().find_child("BindKeyMapping").and_then(|bkp| {
            bkp.find_child("KeyMappingButtonLeft")
                .and_then(|key| key.try_cast::<KeyMappingButton>().ok())
        });
        self.key_mapping_button_right = self.base().find_child("BindKeyMapping").and_then(|bkp| {
            bkp.find_child("KeyMappingButtonRight")
                .and_then(|key| key.try_cast::<KeyMappingButton>().ok())
        });
        self.key_mapping_button_punch_use =
            self.base().find_child("BindKeyMapping").and_then(|bkp| {
                bkp.find_child("KeyMappingButtonPunchUse")
                    .and_then(|key| key.try_cast::<KeyMappingButton>().ok())
            });
        self.player_label = self
            .base()
            .find_child("PlayerLabel")
            .and_then(|pl| pl.try_cast::<RichTextLabel>().ok());
        if self.key_mapping_button_duck.is_some()
            && self.key_mapping_button_jump.is_some()
            && self.key_mapping_button_grab_throw.is_some()
            && self.key_mapping_button_left.is_some()
            && self.key_mapping_button_right.is_some()
            && self.key_mapping_button_punch_use.is_some()
            && self.player_label.is_some()
        {
            godot_print!("player_controls_loaded_correctly");
        }
    }
}

#[godot_api]
impl PlayerControlsPanel {
    pub fn assing_player(&mut self, player_num: u8) {
        self.player_num_assigned = Some(player_num);
        if let Some(ref mut label) = self.player_label {
            label.set_text(&format!("PLAYER {}", player_num.to_string()));
        }
    }
    pub fn all_controls_assigned(&self) -> bool {
        let mut result = true;
        if let Some(ref button_jump) = self.key_mapping_button_jump
            && let Some(ref button_duck) = self.key_mapping_button_duck
            && let Some(ref button_grab_throw) = self.key_mapping_button_grab_throw
            && let Some(ref button_left) = self.key_mapping_button_left
            && let Some(ref button_right) = self.key_mapping_button_right
            && let Some(ref button_punch_use) = self.key_mapping_button_punch_use
        {
            [
                button_duck,
                button_grab_throw,
                button_jump,
                button_left,
                button_punch_use,
                button_right,
            ]
            .iter()
            .for_each(|button| result = result && button.bind().has_key_saved());
        }
        return result;
    }
    pub fn load_control_scheme(&mut self, control: Gd<PlayerControlScheme>) {
        if let Some(ref mut button_jump) = self.key_mapping_button_jump
            && let Some(ref mut button_duck) = self.key_mapping_button_duck
            && let Some(ref mut button_grab_throw) = self.key_mapping_button_grab_throw
            && let Some(ref mut button_left) = self.key_mapping_button_left
            && let Some(ref mut button_right) = self.key_mapping_button_right
            && let Some(ref mut button_punch_use) = self.key_mapping_button_punch_use
        {
            button_jump.bind_mut().save_key(control.bind().jump_key);
            button_duck.bind_mut().save_key(control.bind().duck_key);
            button_left.bind_mut().save_key(control.bind().left_key);
            button_right.bind_mut().save_key(control.bind().right_key);
            button_punch_use
                .bind_mut()
                .save_key(control.bind().punch_use_key);
            button_grab_throw
                .bind_mut()
                .save_key(control.bind().grab_throw_key);
        }
    }
    pub fn asign_controls_to_game_manager(&self) {
        if self.all_controls_assigned()
        //just in case lol
        {
            let gm = self
                .base()
                .try_get_node_as::<GameManager>("/root/GameManager");
            if let Some(mut gm) = gm {
                let mut control_scheme = Gd::from_object(PlayerControlScheme {
                    jump_key: Key::NONE,
                    left_key: Key::NONE,
                    right_key: Key::NONE,
                    duck_key: Key::NONE,
                    punch_use_key: Key::NONE,
                    grab_throw_key: Key::NONE,
                });
                if let Some(ref button_jump) = self.key_mapping_button_jump
                    && let Some(ref button_duck) = self.key_mapping_button_duck
                    && let Some(ref button_grab_throw) = self.key_mapping_button_grab_throw
                    && let Some(ref button_left) = self.key_mapping_button_left
                    && let Some(ref button_right) = self.key_mapping_button_right
                    && let Some(ref button_punch_use) = self.key_mapping_button_punch_use
                {
                    control_scheme.bind_mut().jump_key = button_jump.bind().get_key_saved();
                    control_scheme.bind_mut().left_key = button_left.bind().get_key_saved();
                    control_scheme.bind_mut().right_key = button_right.bind().get_key_saved();
                    control_scheme.bind_mut().duck_key = button_duck.bind().get_key_saved();
                    control_scheme.bind_mut().punch_use_key =
                        button_punch_use.bind().get_key_saved();
                    control_scheme.bind_mut().grab_throw_key =
                        button_grab_throw.bind().get_key_saved();
                }
                if let Some(player_num) = self.player_num_assigned {
                    gm.bind_mut()
                        .save_player_controls(player_num, control_scheme);
                }
            }
        }
    }
}
