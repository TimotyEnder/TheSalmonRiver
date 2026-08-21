use godot::{
    classes::{AudioStreamPlayer2D, INode3D},
    prelude::*,
};

use crate::{
    game_managers::audio_manager::{AudioManager, AudioPlayBuilder},
    game_scripts::game_utils::vec3_to_vec2,
};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct SawBlade {
    base: Base<Node3D>,
    audio_player_handle: Option<Gd<AudioStreamPlayer2D>>,
}

#[godot_api]
impl INode3D for SawBlade {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            audio_player_handle: None,
        }
    }
}

#[godot_api]
impl SawBlade {
    pub fn play_sound(&mut self) {
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(mut audio) = audio {
            self.audio_player_handle = audio.bind_mut().play_sound_build_with_player_handle(
                AudioPlayBuilder::play_sound_effect(crate::sound_utils::SoundEffect::SawBlade)
                    .at_position(vec3_to_vec2(self.base().get_global_position())),
            );
        }
    }
    pub fn stop_sound(&mut self) {
        if let Some(ref mut handle) = self.audio_player_handle {
            handle.call_deferred("queue_free", &[]);
        }
    }
}
