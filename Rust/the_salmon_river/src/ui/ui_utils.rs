use std::str::FromStr;

use godot::{
    classes::{
        class_macros::private::virtuals::ZipReader::{GString, Vector2i},
        display_server::WindowMode,
    },
    meta::{
        Element, FromGodot, GodotConvert, ToGodot, conv::ByValue, error::ConvertError,
        shape::GodotShape,
    },
    register::property::SimpleVar,
};
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
