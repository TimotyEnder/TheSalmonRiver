use godot::{
    classes::{
        Button, IButton, InputEvent, InputEventJoypadButton, InputEventJoypadMotion, InputEventKey,
    },
    global::Key,
    prelude::*,
};

use crate::{
    game_managers::{
        audio_manager::{AudioManager, AudioPlayBuilder},
        input_mapping::InputMapping::{self, GamepadAxis, GamepadButton, Keyboard},
    },
    sound_utils::SoundEffect,
    ui::player_controls_panel::PlayerControlsPanel,
};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct KeyMappingButton {
    base: Base<Button>,
    is_listening: bool,
    input_mapping_saved: Option<InputMapping>,
    parent_panel: Option<Gd<PlayerControlsPanel>>,
}

#[godot_api]
impl IButton for KeyMappingButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            is_listening: false,
            input_mapping_saved: None,
            parent_panel: None,
        }
    }
    fn ready(&mut self) {
        self.base_mut().set_text("ASSIGN");
        let this = self.to_gd();
        self.base()
            .signals()
            .pressed()
            .connect_other(&this, Self::on_click);
    }
    fn input(&mut self, event: Gd<InputEvent>) {
        if self.is_listening {
            if let Some(key_event) = event.clone().try_cast::<InputEventKey>().ok() {
                let key_code = key_event.get_keycode();
                self.save_input_mapping(Keyboard(key_code), false);
            } else if let Some(b) = event.clone().try_cast::<InputEventJoypadButton>().ok() {
                self.save_input_mapping(GamepadButton(b.get_button_index(), b.get_device()), false);
            } else if let Some(m) = event.clone().try_cast::<InputEventJoypadMotion>().ok() {
                if m.get_axis_value().abs() > 0.5 {
                    let value = if m.get_axis_value() > 0.0 { 1.0 } else { -1.0 };
                    self.save_input_mapping(
                        GamepadAxis(m.get_axis(), value, m.get_device()),
                        false,
                    );
                }
            }
        }
    }
}

#[godot_api]
impl KeyMappingButton {
    #[signal]
    pub fn on_key_changed();
    pub fn save_input_mapping(&mut self, input_mapping: InputMapping, from_load: bool) {
        let audio_manager = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        let Some(mut audio) = audio_manager else {
            return;
        };
        if input_mapping != Keyboard(Key::ESCAPE) {
            self.input_mapping_saved = Some(input_mapping);
            if !from_load {
                audio.bind_mut().play_sound_built(
                    AudioPlayBuilder::play_sound_effect(SoundEffect::UIDecrement)
                        .at_position(self.base().get_global_position()),
                );
            }
        } else if !from_load {
            audio.bind_mut().play_sound_built(
                AudioPlayBuilder::play_sound_effect(SoundEffect::UICancel)
                    .at_position(self.base().get_global_position()),
            );
        }
        self.is_listening = false;
        self.set_label_to_input_mapping();
    }
    fn set_label_to_input_mapping(&mut self) {
        if let Some(key_code) = self.input_mapping_saved {
            self.base_mut().set_text(&format!("{:?}", key_code));
            self.signals().on_key_changed().emit();
        }
    }

    fn on_click(&mut self) {
        let id = self.base().instance_id();
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(mut audio) = audio {
            audio.bind_mut().play_sound_built(
                AudioPlayBuilder::play_sound_effect(SoundEffect::UIIncrement)
                    .at_position(self.base().get_global_position()),
            );
        }
        self.is_listening = true;
        self.base_mut().set_text("PRESS");
        if let Some(ref mut parent) = self.parent_panel {
            parent.bind_mut().exclusivety_reset(id);
        }
    }
    pub fn get_input_mapping_saved(&self) -> InputMapping {
        if let Some(input_mapping) = self.input_mapping_saved {
            return input_mapping;
        } else {
            return Keyboard(Key::NONE);
        }
    }
    pub fn has_input_mapping_saved(&self) -> bool {
        match self.input_mapping_saved {
            Some(InputMapping::Keyboard(key)) => key != Key::NONE,
            Some(_) => true,
            None => false,
        }
    }
    pub fn exclusivity_reset(&mut self) {
        self.is_listening = false;
        self.set_label_to_input_mapping();
    }
    pub fn set_parent_control_panel(&mut self, panel: Gd<PlayerControlsPanel>) {
        self.parent_panel = Some(panel);
    }
}
