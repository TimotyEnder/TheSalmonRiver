use godot::{
    classes::{AudioListener2D, AudioStream, AudioStreamPlayer2D, IAudioListener2D},
    prelude::*,
};

use crate::sound_utils::SoundEffect;

#[derive(GodotClass)]
#[class(base=AudioListener2D)]
pub struct AudioManager {
    base: Base<AudioListener2D>,
}

#[godot_api]
impl IAudioListener2D for AudioManager {
    fn init(base: Base<AudioListener2D>) -> Self {
        Self { base }
    }
    fn ready(&mut self) {}
}

#[godot_api]
impl AudioManager {
    #[func]
    pub fn play_sound(&mut self, effect: SoundEffect, position: Vector2) {
        let stream: Gd<AudioStream> = load(effect.to_sound_effect_path());
        let mut player = AudioStreamPlayer2D::new_alloc();
        player.set_stream(&stream);
        player.set_position(position);
        Signal::from_object_signal(&player, "finished").connect(&player.callable("queue_free"));
        self.base()
            .get_tree()
            .get_current_scene()
            .and_then(|mut root| Some(root.add_child(&player)));
        player.play();
    }
}
