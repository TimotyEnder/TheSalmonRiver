use godot::{
    classes::{Button, IButton},
    prelude::*,
};

use crate::{game_managers::audio_manager::AudioManager, game_scripts::game_utils::vec3_to_vec2};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct SceneChangingButton {
    base: Base<Button>,
    #[export]
    scene_name_to_change_to: StringName,
}

#[godot_api]
impl IButton for SceneChangingButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            scene_name_to_change_to: StringName::from(""),
        }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        self.signals()
            .button_down()
            .connect_other(&this, Self::scene_change);
    }
}

#[godot_api]
impl SceneChangingButton {
    fn scene_change(&mut self) {
        let scene_name = format!("res://Scenes/{}.tscn", self.scene_name_to_change_to);
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(mut audio) = audio {
            audio.bind_mut().play_sound(
                crate::sound_utils::SoundEffect::UIAccept,
                self.base().get_global_position(),
            );
        }
        self.base().get_tree().change_scene_to_file(&scene_name);
    }
}
