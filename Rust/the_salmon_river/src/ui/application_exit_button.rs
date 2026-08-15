use godot::{
    classes::{Button, IButton},
    prelude::*,
};

use crate::{game_managers::audio_manager::AudioManager, sound_utils::SoundEffect};

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
            .button_down()
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
            audio
                .bind_mut()
                .play_sound(SoundEffect::UICancel, self.base().get_position());
        }
        self.base_mut().get_tree().quit();
    }
}
