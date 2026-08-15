use godot::{
    classes::{DisplayServer, IPanel, Panel},
    prelude::*,
};

use crate::ui::{
    string_label_selector::StringLabelSelector,
    ui_utils::{Resolution, parse_window_mode_from_string},
};

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct SettingsPanel {
    base: Base<Panel>,
    sc_resolution: Option<Gd<StringLabelSelector>>,
    sc_window_type: Option<Gd<StringLabelSelector>>,
}

#[godot_api]
impl IPanel for SettingsPanel {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            sc_resolution: None,
            sc_window_type: None,
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

        self.change_settings();
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
        {
            let resolution = Resolution::parse_from_string(resolution.bind().get_value());
            let window_type = parse_window_mode_from_string(window_type.bind().get_value());
            DisplayServer::singleton().window_set_mode(window_type);
            DisplayServer::singleton().window_set_size(Vector2i {
                x: resolution.width,
                y: resolution.height,
            });
        }
    }
}
