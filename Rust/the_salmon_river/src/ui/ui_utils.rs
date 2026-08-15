use godot::classes::{
    class_macros::private::virtuals::ZipReader::GString, display_server::WindowMode,
};

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
}
pub fn parse_window_mode_from_string(str: GString) -> WindowMode {
    if str.contains("FULLSCREEN") {
        return WindowMode::FULLSCREEN;
    } else if str.contains("MAX") {
        return WindowMode::MAXIMIZED;
    } else {
        return WindowMode::WINDOWED;
    }
}
