use godot::{
    classes::{Area3D, GpuParticles3D, IStaticBody3D, PhysicsRayQueryParameters3D, StaticBody3D},
    prelude::*,
};

use crate::{game_managers::audio_manager::AudioManager, game_scripts::game_utils::vec3_to_vec2};

#[derive(GodotClass)]
#[class(base=StaticBody3D)]
pub struct River {
    base: Base<StaticBody3D>,
    #[export]
    splash_area: Option<Gd<Area3D>>,
}

#[godot_api]
impl IStaticBody3D for River {
    fn init(base: Base<StaticBody3D>) -> Self {
        Self {
            base,
            splash_area: None,
        }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        if let Some(ref mut splash_area) = self.splash_area {
            splash_area
                .signals()
                .area_entered()
                .connect_other(&this, Self::create_water_splash);
            splash_area
                .signals()
                .area_exited()
                .connect_other(&this, Self::create_water_splash);
        }
    }
}

#[godot_api]
impl River {
    pub fn create_water_splash(&mut self, body: Gd<Area3D>) {
        if body.get_name().contains("Throwable") {
            self.intersection_splash(body);
        } else {
            let spawn_position = Vector3::new(
                body.get_global_position().x,
                self.base().get_global_position().y + 1.0,
                body.get_global_position().z,
            );
            self.spawn_splash_on_position(spawn_position);
        }
    }
    fn intersection_splash(&mut self, body: Gd<Area3D>) {
        let Some(mut space_state) = self
            .base()
            .get_world_3d()
            .and_then(|world| world.get_direct_space_state())
        else {
            return;
        };
        let from = self.base().get_global_position();
        let to = body.get_global_position();
        let Some(mut query) = PhysicsRayQueryParameters3D::create(from, to) else {
            return;
        };
        query.set_collide_with_areas(true);
        query.set_collide_with_bodies(true);
        let result = space_state.intersect_ray(&query);
        let Some(contact_point) = result
            .get("position")
            .and_then(|pos| Some(pos.to::<Vector3>()))
        else {
            return;
        };
        self.spawn_splash_on_position(contact_point);
    }
    fn spawn_splash_on_position(&mut self, pos: Vector3) {
        let scene = load::<PackedScene>("res://Prefabs/water_splash.tscn");
        if let Some(mut water_splash) = scene
            .instantiate()
            .and_then(|splash| splash.try_cast::<Node3D>().ok())
        {
            water_splash.set_position(pos);
            self.base()
                .get_tree()
                .get_current_scene()
                .and_then(|mut root| Some(root.add_child(&water_splash)));
            water_splash.find_child("SplashParticles").and_then(|sp| {
                sp.try_cast::<GpuParticles3D>()
                    .ok()
                    .and_then(|mut sp| Some(sp.set_emitting(true)))
            });
            let audio = self
                .base()
                .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
            if let Some(mut audio) = audio {
                audio.bind_mut().play_sound_debounced_randomized_pitch(
                    crate::sound_utils::SoundEffect::WaterSplash,
                    vec3_to_vec2(pos),
                );
            }
            let timer = self.base().get_tree().create_timer(2.0);
            godot::task::spawn(async move {
                Signal::from_object_signal(&timer, "timeout")
                    .to_future::<()>()
                    .await;
                water_splash.queue_free();
            });
        }
    }
}
