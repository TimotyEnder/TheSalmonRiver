use godot::{
    classes::{DisplayServer, IPanel, Panel, Slider},
    prelude::*,
};

use crate::{
    game_managers::{audio_manager::AudioManager, game_manager::GameManager},
    settings_state::SettingsState,
    ui::{
        string_label_selector::StringLabelSelector,
        ui_utils::{Resolution, parse_window_mode_from_string, parse_window_mode_to_string},
    },
};

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct SettingsPanel {
    base: Base<Panel>,
    sc_resolution: Option<Gd<StringLabelSelector>>,
    sc_window_type: Option<Gd<StringLabelSelector>>,
    slider_game_volume: Option<Gd<Slider>>,
}

#[godot_api]
impl IPanel for SettingsPanel {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            sc_resolution: None,
            sc_window_type: None,
            slider_game_volume: None,
        }
    }
    fn ready(&mut self) {
        self.sc_resolution = self
            .base()
            .find_child("StringSelectorResolution")
            .and_then(|sc| sc.try_cast::<StringLabelSelector>().ok());
        self.sc_window_type = self
            .base()
            .find_child("StringSelectorWindowType")
            .and_then(|sc| sc.try_cast::<StringLabelSelector>().ok());
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
        if let Some(mut gm) = gm
            && let Some(ref mut window_type) = self.sc_window_type
            && let Some(ref mut resolution) = self.sc_resolution
        {
            let settings = gm.bind_mut().request_settings_state();
            if resolution
                .bind_mut()
                .load_value(settings.bind().resolution.to_string())
                && window_type
                    .bind_mut()
                    .load_value(parse_window_mode_to_string(settings.bind().window_mode))
            {
                self.change_settings();
            }
        }
        let this = self.to_gd();
        if let Some(ref mut window_type) = self.sc_window_type
            && let Some(ref mut resolution) = self.sc_resolution
        {
            window_type
                .signals()
                .value_changed()
                .connect_other(&this, Self::change_settings);
            resolution
                .signals()
                .value_changed()
                .connect_other(&this, Self::change_settings);
        }
    }
}

#[godot_api]
impl SettingsPanel {
    fn change_settings(&mut self) {
        if let Some(ref mut window_type) = self.sc_window_type
            && let Some(ref mut resolution) = self.sc_resolution
            && let Some(slider) = self.slider_game_volume
        {
            let resolution = Resolution::parse_from_string(resolution.bind().get_value());
            let window_mode = parse_window_mode_from_string(window_type.bind().get_value());
            let audio = self
                .base()
                .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
            if let Some(mut audio) = audio {
                audio.bind_mut().set_game_volume(slider.get_value() as f32);
            }
            DisplayServer::singleton().window_set_mode(window_mode);
            DisplayServer::singleton().window_set_size(Vector2i {
                x: resolution.width,
                y: resolution.height,
            });
            let gm = self
                .base()
                .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
            if let Some(mut gm) = gm {
                let mut settings_to_save = Gd::from_object(SettingsState::default());
                settings_to_save.bind_mut().resolution = resolution;
                settings_to_save.bind_mut().window_mode = window_mode;
                settings_to_save.bind_mut().game_volume_percent = slider.get_value() as f32;
                gm.bind_mut().save_settings_state(settings_to_save);
            }
        }
    }
}
