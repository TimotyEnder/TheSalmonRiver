use godot::{
    classes::{INode3D, RandomNumberGenerator},
    prelude::*,
};

use crate::{
    game_managers::audio_manager::{AudioManager, AudioPlayBuilder},
    game_scripts::game_utils::vec3_to_vec2,
    sound_utils::SoundEffect::WingFlapDuck,
};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct FlyingDuck {
    base: Base<Node3D>,
    play_flap_sounds: bool,
}

#[godot_api]
impl INode3D for FlyingDuck {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            play_flap_sounds: false,
        }
    }
}

#[godot_api]
impl FlyingDuck {
    pub fn set_play_flap(&mut self, status: bool) {
        self.play_flap_sounds = status;
    }
    #[func]
    pub fn play_wing_flap_sound(&mut self) {
        if self.play_flap_sounds {
            let audio = self
                .base()
                .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
            if let Some(mut audio) = audio {
                let mut rand = RandomNumberGenerator::new_gd();
                self.base()
                    .get_tree()
                    .create_timer(rand.randf_range(0.0, 1.0) as f64);
                let sound_pos = self.base().get_global_position();
                godot::task::spawn(async move {
                    audio.bind_mut().play_sound_built(
                        AudioPlayBuilder::play_sound_effect(WingFlapDuck)
                            .at_position(vec3_to_vec2(sound_pos))
                            .debounced(),
                    );
                });
            }
        }
    }
}
