use godot::{
    classes::{Button, IButton},
    prelude::*,
};

use crate::{game_managers::audio_manager::AudioManager, ui::pregame_panel::PregamePanel};
#[derive(GodotClass)]
#[class(base=Button)]

pub struct MainMenuStartMatchButton {
    base: Base<Button>,
    #[export]
    pregame_panel: Option<Gd<PregamePanel>>,
}
#[godot_api]
impl IButton for MainMenuStartMatchButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            pregame_panel: None,
        }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        self.base()
            .signals()
            .pressed()
            .connect_other(&this, Self::on_pressed);
    }
}

#[godot_api]
impl MainMenuStartMatchButton {
    #[func]
    fn on_pressed(&mut self) {
        let mut tree = self.base().get_tree();
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(ref mut pregame) = self.pregame_panel {
            if pregame.bind_mut().start_game()
                && let Some(mut audio) = audio
            {
                audio.call_deferred(
                    "play_sound",
                    &[
                        crate::sound_utils::SoundEffect::UIAccept.to_variant(),
                        self.base().get_position().to_variant(),
                    ],
                );
                //tree.change_scene_to_file("res://Scenes/victory_screen.tscn"); //debug
                tree.change_scene_to_file("res://Scenes/main.tscn");
            }
        }
    }
}
