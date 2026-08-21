use godot::{
    classes::{AudioStream, AudioStreamPlayer2D, RandomNumberGenerator, Time},
    prelude::*,
};

use crate::sound_utils::SoundEffect;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct AudioManager {
    base: Base<Node2D>,
    next_effect_time_ms: u64,
    debounce_duration_ms: u64,
}

#[godot_api]
impl INode2D for AudioManager {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base,
            next_effect_time_ms: 0,
            debounce_duration_ms: 100,
        }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        self.base()
            .get_tree()
            .signals()
            .scene_changed()
            .connect_other(&this, Self::wipe_zombie_audio_players);
    }
}

impl AudioManager {
    pub fn wipe_zombie_audio_players(&mut self) {
        self.base_mut()
            .get_children()
            .iter_shared()
            .for_each(|child| {
                if let Some(mut audio_player) = child.try_cast::<AudioStreamPlayer2D>().ok() {
                    audio_player.call_deferred("queue_free", &[]);
                }
            });
    }
    pub fn play_sound_built(&mut self, builder: AudioPlayBuilder) {
        let now = Time::singleton().get_ticks_msec();
        if builder.debounced {
            if now <= self.next_effect_time_ms {
                return;
            }
            self.next_effect_time_ms = now + self.debounce_duration_ms;
        }

        let stream: Gd<AudioStream> = load(builder.sound_effect.to_sound_effect_path());
        let mut player = AudioStreamPlayer2D::new_alloc();
        player.set_stream(&stream);
        player.set_position(builder.position);
        Signal::from_object_signal(&player, "finished").connect(&player.callable("queue_free"));
        if builder.randomise_pitch {
            let mut random = RandomNumberGenerator::new_gd();
            player.set_pitch_scale(
                random.randf_range(builder.randomise_range_from, builder.randomise_range_to),
            );
        }
        player.set_volume_db(builder.volume);
        self.base_mut().add_child(&player);
        player.play();
    }
    pub fn play_sound_build_with_player_handle(
        &mut self,
        builder: AudioPlayBuilder,
    ) -> Option<Gd<AudioStreamPlayer2D>> {
        let now = Time::singleton().get_ticks_msec();
        if builder.debounced {
            if now <= self.next_effect_time_ms {
                return None;
            }
            self.next_effect_time_ms = now + self.debounce_duration_ms;
        }

        let stream: Gd<AudioStream> = load(builder.sound_effect.to_sound_effect_path());
        let mut player = AudioStreamPlayer2D::new_alloc();
        player.set_stream(&stream);
        player.set_position(builder.position);
        if builder.randomise_pitch {
            let mut random = RandomNumberGenerator::new_gd();
            player.set_pitch_scale(
                random.randf_range(builder.randomise_range_from, builder.randomise_range_to),
            );
        }
        player.set_volume_db(builder.volume);
        self.base_mut().add_child(&player);
        player.play();
        Some(player)
    }
}

pub struct AudioPlayBuilder {
    sound_effect: SoundEffect,
    volume: f32,
    randomise_pitch: bool,
    randomise_range_from: f32,
    randomise_range_to: f32,
    debounced: bool,
    position: Vector2,
}

impl AudioPlayBuilder {
    pub fn play_sound_effect(sound_effect: SoundEffect) -> Self {
        Self {
            sound_effect,
            volume: 1.0,
            randomise_pitch: false,
            randomise_range_from: 0.9,
            randomise_range_to: 1.1,
            debounced: false,
            position: Vector2::ZERO,
        }
    }
    pub fn with_volume(mut self, volume_db: f32) -> Self {
        self.volume = volume_db;
        self
    }
    pub fn with_randomized_pitch_range(mut self, from: f32, to: f32) -> Self {
        self.randomise_pitch = true;
        self.randomise_range_from = from;
        self.randomise_range_to = to;
        self
    }
    pub fn debounced(mut self) -> Self {
        self.debounced = true;
        self
    }
    pub fn at_position(mut self, pos: Vector2) -> Self {
        self.position = pos;
        self
    }
}
