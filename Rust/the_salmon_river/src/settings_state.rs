use std::str::FromStr;

use godot::classes::IRefCounted;
use godot::classes::display_server::WindowMode;
use godot::prelude::*;

use crate::ui::ui_utils::{Resolution, parse_window_mode_from_string, parse_window_mode_to_string};

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct SettingsState {
    #[var]
    pub resolution: Resolution,
    #[var]
    pub window_mode: WindowMode,
}

#[godot_api]
impl IRefCounted for SettingsState {
    fn init(_base: Base<RefCounted>) -> Self {
        Self {
            resolution: Resolution {
                width: 1980,
                height: 1080,
            },
            window_mode: WindowMode::FULLSCREEN,
        }
    }
}
impl Default for SettingsState {
    fn default() -> Self {
        Self {
            resolution: Resolution::default(),
            window_mode: WindowMode::FULLSCREEN,
        }
    }
}
impl SettingsState {
    pub fn to_dict(&self) -> Dictionary<StringName, Variant> {
        let mut dict: Dictionary<StringName, Variant> = Dictionary::new();
        dict.set("resolution", &self.resolution.to_string().to_variant());
        dict.set(
            "window_mode",
            &parse_window_mode_to_string(self.window_mode).to_variant(),
        );
        dict
    }
    pub fn from_dict(dict: Dictionary<Variant, Variant>) -> Self {
        Self {
            resolution: Resolution::parse_from_string(
                dict.get(&GString::from("resolution"))
                    .map(string_from_variant)
                    .unwrap_or_default(),
            ),
            window_mode: parse_window_mode_from_string(
                dict.get(&GString::from("window_mode"))
                    .map(string_from_variant)
                    .unwrap_or_default(),
            ),
        }
    }
}
fn string_from_variant(value: Variant) -> GString {
    value.try_to::<GString>().unwrap_or_default()
}
