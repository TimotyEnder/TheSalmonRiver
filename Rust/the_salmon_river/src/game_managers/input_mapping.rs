use std::fmt::Debug;

use godot::{
    classes::Input,
    global::{JoyAxis, JoyButton, Key},
    obj::Singleton,
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
                    input.get_joy_axis(*device_id, *axis) > *value
                } else {
                    input.get_joy_axis(*device_id, *axis) < *value
                };
            }
            InputMapping::GamepadButton(button, device_id) => {
                return input.is_joy_button_pressed(*device_id, *button);
            }
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
