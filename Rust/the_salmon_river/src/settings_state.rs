use godot::classes::IRefCounted;
use godot::classes::display_server::WindowMode;
use godot::prelude::*;

use crate::ui::ui_utils::Resolution;

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
            resolution: Resolution {
                width: 1920,
                height: 1080,
            },
            window_mode: WindowMode::FULLSCREEN,
        }
    }
}
