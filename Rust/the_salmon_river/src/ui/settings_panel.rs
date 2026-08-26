use godot::{
    classes::{Button, DisplayServer, IPanel, Panel, Slider},
    prelude::*,
};

use crate::{
    game_managers::{audio_manager::AudioManager, game_manager::GameManager},
    settings_state::SettingsState,
    ui::{
        settings_button::SettingsButton,
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
    focus_grab_flag: bool,
    settings_button: Option<Gd<SettingsButton>>,
}

#[godot_api]
impl IPanel for SettingsPanel {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            sc_resolution: None,
            sc_window_type: None,
            slider_game_volume: None,
            focus_grab_flag: false,
            settings_button: None,
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
        self.slider_game_volume = self
            .base()
            .find_child("VolumeSlider")
            .and_then(|slider| slider.try_cast::<Slider>().ok());
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
        if let Some(mut gm) = gm
            && let Some(ref mut window_type) = self.sc_window_type
            && let Some(ref mut resolution) = self.sc_resolution
            && let Some(ref mut slider) = self.slider_game_volume
        {
            let settings = gm.bind_mut().request_settings_state();
            if resolution
                .bind_mut()
                .load_value(settings.bind().resolution.to_string())
                && window_type
                    .bind_mut()
                    .load_value(parse_window_mode_to_string(settings.bind().window_mode))
            {
                slider.set_value(settings.bind().game_volume_percent as f64);
                self.change_settings();
            }
        }
        let this = self.to_gd();
        if let Some(ref mut window_type) = self.sc_window_type
            && let Some(ref mut resolution) = self.sc_resolution
            && let Some(ref mut slider) = self.slider_game_volume
        {
            window_type
                .signals()
                .value_changed()
                .connect_other(&this, Self::change_settings);
            resolution
                .signals()
                .value_changed()
                .connect_other(&this, Self::change_settings);
            slider
                .signals()
                .drag_ended()
                .connect_other(&this, Self::volume_slider_changed);
        }
    }
    fn process(&mut self, _delta: f32) {
        if self.base().is_visible() && !self.focus_grab_flag {
            self.focus_grab_flag = true;
            self.focus_first_button_child();
        }
        if !self.base().is_visible() && self.focus_grab_flag {
            self.focus_grab_flag = false;
            self.reestablish_focus_back();
        }
    }
}

#[godot_api]
impl SettingsPanel {
    pub fn set_settings_button(&mut self, button: Gd<SettingsButton>) {
        self.settings_button = Some(button);
    }
    fn focus_first_button_child(&mut self) {
        for child in self
            .base()
            .get_children_ex()
            .include_internal(true)
            .done()
            .iter_shared()
        {
            let cast_attempt = child.try_cast::<Button>().ok();
            if let Some(mut button) = cast_attempt {
                button.grab_focus();
            }
        }
    }
    fn reestablish_focus_back(&mut self) {
        if let Some(ref mut settings_button) = self.settings_button {
            settings_button.grab_focus();
        }
    }
    fn volume_slider_changed(&mut self, value_changed: bool) {
        if value_changed {
            self.change_settings();
        }
    }
    fn change_settings(&mut self) {
        godot_print!("Settings Changing");
        if let Some(ref mut window_type) = self.sc_window_type
            && let Some(ref mut resolution) = self.sc_resolution
            && let Some(ref slider) = self.slider_game_volume
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
