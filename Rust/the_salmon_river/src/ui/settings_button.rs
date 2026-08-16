use godot::{
    classes::{Button, CanvasLayer, IButton, Panel},
    prelude::*,
};

use crate::game_managers::audio_manager::AudioManager;

#[derive(GodotClass)]
#[class(base=Button)]
pub struct SettingsButton {
    base: Base<Button>,
    settings_panel: Option<Gd<Panel>>,
}

#[godot_api]
impl IButton for SettingsButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            settings_panel: None,
        }
    }
    fn ready(&mut self) {
        let settings_panel_scene = load::<PackedScene>("res://Prefabs/Ui/settings_panel.tscn");
        if let Some(settings) = settings_panel_scene
            .instantiate()
            .and_then(|panel| panel.try_cast::<CanvasLayer>().ok())
        {
            self.base()
                .get_tree()
                .get_current_scene()
                .and_then(|ref mut root| {
                    Some(root.call_deferred("add_child", &[settings.to_variant()]))
                });
            self.settings_panel = settings
                .find_child("SettingsPanel")
                .and_then(|panel| panel.try_cast::<Panel>().ok());
            if let Some(ref mut settings) = self.settings_panel {
                settings.set_visible(false);
            }
        }
        let this = self.to_gd();
        self.signals()
            .button_down()
            .connect_other(&this, Self::on_click);
    }
}

#[godot_api]
impl SettingsButton {
    fn on_click(&mut self) {
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(mut audio) = audio {
            audio.bind_mut().play_sound(
                crate::sound_utils::SoundEffect::UIAccept,
                self.base().get_global_position(),
            );
        }
        if let Some(ref mut settings) = self.settings_panel {
            settings.set_visible(true);
        }
    }
}
