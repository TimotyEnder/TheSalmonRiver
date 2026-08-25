use std::fmt::Debug;

use godot::{
    classes::Input,
    global::{JoyAxis, JoyButton, Key},
    obj::Singleton,
    prelude::*,
};

#[derive(Copy, Clone, PartialEq)]
pub enum InputMapping {
    Keyboard(Key),
    GamepadButton(JoyButton, i32),  //button index, device_id
    GamepadAxis(JoyAxis, f32, i32), //axis , axis value, device_id
}
impl InputMapping {
    pub fn is_actuated(&self) -> bool {
        let input = Input::singleton();
        match self {
            InputMapping::Keyboard(key) => return input.is_key_label_pressed(*key),
            InputMapping::GamepadAxis(axis, value, device_id) => {
                return if *value > 0.0 {
                    input.get_joy_axis(*device_id, *axis) >= *value
                } else {
                    input.get_joy_axis(*device_id, *axis) <= *value
                };
            }
            InputMapping::GamepadButton(button, device_id) => {
                return input.is_joy_button_pressed(*device_id, *button);
            }
        }
    }
    pub fn to_dict(&self) -> Dictionary<Variant, Variant> {
        let mut dict = Dictionary::new();
        match self {
            InputMapping::Keyboard(key) => {
                dict.set("type", "keyboard");
                dict.set("key", &(*key).ord().to_variant());
            }
            InputMapping::GamepadButton(button, device) => {
                dict.set("type", "gamepad_button");
                dict.set("button", &(*button).ord().to_variant());
                dict.set("device", &(*device).to_variant());
            }
            InputMapping::GamepadAxis(axis, value, device) => {
                dict.set("type", "gamepad_axis");
                dict.set("axis", &(*axis).ord().to_variant());
                dict.set("value", &(*value).to_variant());
                dict.set("device", &(*device).to_variant());
            }
        }
        dict
    }
    /* fn key_from_variant(value: Variant) -> Key {
    let ord = value
        .try_to::<i64>()
        .or_else(|_| value.try_to::<f64>().map(|f| f as i64))
        .unwrap_or(0);
    Key::try_from_ord(ord as i32).unwrap_or(Key::NONE)*/
    pub fn from_dict(dict: Dictionary<Variant, Variant>) -> Self {
        let Some(type_str) = dict.get("type").and_then(|v| v.try_to::<GString>().ok()) else {
            return InputMapping::Keyboard(Key::NONE);
        };
        match type_str.to_string().as_str() {
            "keyboard" => {
                let value = dict.get("key").unwrap_or_default();
                let key_ord = value
                    .try_to::<i64>()
                    .or_else(|_| value.try_to::<f64>().map(|f| f as i64))
                    .unwrap_or(0);
                InputMapping::Keyboard(Key::from_ord(key_ord as i32))
            }
            "gamepad_button" => {
                let button_ord = dict
                    .get("button")
                    .and_then(|v| v.try_to::<i32>().ok())
                    .unwrap_or(0);
                let value = dict.get("device").unwrap_or_default();
                let device = value
                    .try_to::<i64>()
                    .or_else(|_| value.try_to::<f64>().map(|f| f as i64))
                    .unwrap_or(0);
                InputMapping::GamepadButton(JoyButton::from_ord(button_ord), device as i32)
            }
            "gamepad_axis" => {
                let axis_ord = dict
                    .get("axis")
                    .and_then(|v| v.try_to::<i32>().ok())
                    .unwrap_or(0);
                let value = dict
                    .get("value")
                    .and_then(|v| v.try_to::<f32>().ok())
                    .unwrap_or(0.0);
                let device_value = dict.get("device").unwrap_or_default();
                let device = device_value
                    .try_to::<i64>()
                    .or_else(|_| device_value.try_to::<f64>().map(|f| f as i64))
                    .unwrap_or(0);
                InputMapping::GamepadAxis(JoyAxis::from_ord(axis_ord), value, device as i32)
            }
            _ => InputMapping::Keyboard(Key::NONE),
        }
    }
}
impl Debug for InputMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyboard(arg0) => f.debug_tuple("").field(arg0).finish(),
            Self::GamepadButton(arg0, arg1) => f
                .debug_tuple("")
                .field(arg0)
                .field(&format!("Gp#:{}", arg1))
                .finish(),
            Self::GamepadAxis(arg0, arg1, arg2) => f
                .debug_tuple("")
                .field(arg0)
                .field(arg1)
                .field(&format!("Gp#:{}", arg2))
                .finish(),
        }
    }
}
impl ToString for InputMapping {
    fn to_string(&self) -> String {
        match self {
            Self::Keyboard(key) => format!("{:?}", key),
            Self::GamepadButton(button, device_id) => format!("Gmpd#{} {:?}", device_id, button),
            Self::GamepadAxis(axis, value, device_id) => match *axis {
                JoyAxis::LEFT_X => format!("Gmpd#{} (L) {}", device_id, {
                    if *value > 0.0 { "Right" } else { "Left" }
                }),
                JoyAxis::LEFT_Y => format!("Gmpd#{} (L) {}", device_id, {
                    if *value > 0.0 { "Down" } else { "Up" }
                }),
                JoyAxis::RIGHT_X => format!("Gmpd#{} (R) {}", device_id, {
                    if *value > 0.0 { "Right" } else { "Left" }
                }),
                JoyAxis::RIGHT_Y => format!("Gmpd#{} (R) {}", device_id, {
                    if *value > 0.0 { "Down" } else { "Up" }
                }),
                _ => format!("Gmpd#{} {:?}", device_id, axis),
            },
        }
    }
}
