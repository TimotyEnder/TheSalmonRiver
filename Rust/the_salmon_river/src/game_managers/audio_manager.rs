use godot::{
    classes::{AudioStream, AudioStreamPlayer2D, RandomNumberGenerator, Time},
    prelude::*,
};

use crate::sound_utils::SoundEffect;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct AudioManager {
    base: Base<Node2D>,
    next_effect_time: u64,
    debounce_duration: u64,
}

#[godot_api]
impl INode2D for AudioManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            next_effect_time: 0,
            debounce_duration: 200,
        }
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
        self.base_mut().add_child(&player);
        player.play();
    }
    #[func]
    pub fn play_sound_randomized_pitch(&mut self, effect: SoundEffect, position: Vector2) {
        let stream: Gd<AudioStream> = load(effect.to_sound_effect_path());
        let mut player = AudioStreamPlayer2D::new_alloc();
        player.set_stream(&stream);
        player.set_position(position);
        Signal::from_object_signal(&player, "finished").connect(&player.callable("queue_free"));
        self.base_mut().add_child(&player);
        let mut random = RandomNumberGenerator::new_gd();
        player.set_pitch_scale(random.randf_range(0.9, 1.1));
        player.play();
        player.set_pitch_scale(1.0);
    }
    #[func]
    pub fn play_sound_debounced(&mut self, effect: SoundEffect, position: Vector2) {
        let time = Time::singleton();
        if time.get_ticks_msec() > self.next_effect_time {
            self.play_sound(effect, position);
            self.next_effect_time = time.get_ticks_msec() + self.debounce_duration;
        }
    }
    #[func]
    pub fn play_sound_debounced_randomized_pitch(
        &mut self,
        effect: SoundEffect,
        position: Vector2,
    ) {
        let time = Time::singleton();
        if time.get_ticks_msec() > self.next_effect_time {
            self.play_sound_randomized_pitch(effect, position);
            self.next_effect_time = time.get_ticks_msec() + self.debounce_duration;
        }
    }
}
