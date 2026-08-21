use godot::{
    classes::{Button, IButton},
    prelude::*,
};

use crate::{
    game_managers::audio_manager::{AudioManager, AudioPlayBuilder},
    sound_utils::SoundEffect,
};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct ApplicationExitButton {
    base: Base<Button>,
}

#[godot_api]
impl IButton for ApplicationExitButton {
    fn init(base: Base<Button>) -> Self {
        Self { base }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        self.base()
            .signals()
            .button_up()
            .connect_other(&this, Self::application_exit);
    }
}

#[godot_api]
impl ApplicationExitButton {
    fn application_exit(&mut self) {
        let audio_manager = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(mut audio) = audio_manager {
            audio.bind_mut().play_sound_built(
                AudioPlayBuilder::play_sound_effect(SoundEffect::UICancel)
                    .at_position(self.base().get_global_position()),
            );
        }
        self.base_mut().get_tree().call_deferred("quit", &[]);
    }
}
