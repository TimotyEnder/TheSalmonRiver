use godot::classes::{Control, RichTextLabel};
use godot::prelude::*;
use godot::{
    classes::{
        Button, Node, PackedScene,
        class_macros::private::virtuals::{
            Xrvrs::Gd,
            ZipReader::{GString, Vector2i},
        },
        display_server::WindowMode,
    },
    meta::{
        Element, FromGodot, GodotConvert, ToGodot, conv::ByValue, error::ConvertError,
        shape::GodotShape,
    },
    register::property::SimpleVar,
};
use std::str::FromStr;

use crate::game_managers::audio_manager::{AudioManager, AudioPlayBuilder};
use crate::sound_utils::SoundEffect::UIerrorSound;
use crate::ui::popup_dialog_button::PopupDialogButton;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    pub width: i32,
    pub height: i32,
}
impl Resolution {
    pub fn parse_from_string(str: GString) -> Self {
        let width_height = str.split("x");
        Self {
            width: width_height[0].to_float() as i32,
            height: width_height[1].to_float() as i32,
        }
    }
    pub fn to_string(&self) -> GString {
        return GString::from_str(&format!("{}x{}", self.width, self.height)).unwrap_or_default();
    }
}
pub fn parse_window_mode_from_string(str: GString) -> WindowMode {
    if str.contains("EXCLUSIVE FULLSCREEN") {
        return WindowMode::EXCLUSIVE_FULLSCREEN;
    } else if str.contains("MAXIMIZED") {
        return WindowMode::MAXIMIZED;
    } else if str.contains("FULLSCREEN") {
        return WindowMode::FULLSCREEN;
    } else {
        return WindowMode::WINDOWED;
    }
}
pub fn parse_window_mode_to_string(window_mode: WindowMode) -> GString {
    let ans = match window_mode {
        WindowMode::EXCLUSIVE_FULLSCREEN => GString::from_str("EXCLUSIVE FULLSCREEN"),
        WindowMode::FULLSCREEN => GString::from_str("FULLSCREEN"),
        WindowMode::WINDOWED => GString::from_str("WINDOWED"),
        WindowMode::MAXIMIZED => GString::from_str("MAXIMIZED"),
        _ => GString::from_str("UNKNOWN"),
    };
    return ans.unwrap_or(GString::new());
}
impl GodotConvert for Resolution {
    type Via = Vector2i;

    fn godot_shape() -> godot::meta::shape::GodotShape {
        GodotShape::of_builtin::<Self::Via>()
    }
}
impl ToGodot for Resolution {
    type Pass = ByValue;

    fn to_godot(&self) -> Vector2i {
        Vector2i::new(self.width, self.height)
    }
}

impl FromGodot for Resolution {
    fn try_from_godot(via: Vector2i) -> Result<Self, ConvertError> {
        Ok(Self {
            width: via.x,
            height: via.y,
        })
    }
}
impl Element for Resolution {}
impl SimpleVar for Resolution {}
impl Default for Resolution {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }
}

pub fn distinct_elements_in_vec(teams: &Vec<u8>) -> bool {
    return teams.iter().any(|element| *element != teams[0]);
}
pub fn spawn_popus_dialog(
    mut scene_root: Gd<Node>,
    button_to_refocus_after_close: Gd<Button>,
    dialog_text: &'static str,
) {
    let scene = load::<PackedScene>("res://Prefabs/Ui/popup_dialog.tscn");
    let audio = scene_root.try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");

    if let Some(popup) = scene
        .instantiate()
        .and_then(|popup_root| popup_root.try_cast::<Control>().ok())
        && let Some(mut audio) = audio
    {
        scene_root.add_child(&popup);
        audio.bind_mut().play_sound_built(
            AudioPlayBuilder::play_sound_effect(UIerrorSound).at_position(Vector2::ZERO),
        );
        let Some(mut text) = popup.find_child("Panel").and_then(|panel| {
            panel
                .find_child("Text")
                .and_then(|txt| txt.try_cast::<RichTextLabel>().ok())
        }) else {
            return;
        };
        let Some(mut button) = popup.find_child("Panel").and_then(|panel| {
            panel
                .find_child("Button")
                .and_then(|txt| txt.try_cast::<PopupDialogButton>().ok())
        }) else {
            return;
        };
        text.set_text(dialog_text);
        button
            .bind_mut()
            .assign_refocus_button(button_to_refocus_after_close);
    }
}
