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
    pub fn from_dict(dict: Dictionary<Variant, Variant>) -> Self {
        let Some(type_str) = dict.get("type").and_then(|v| v.try_to::<GString>().ok()) else {
            return InputMapping::Keyboard(Key::NONE);
        };
        match type_str.to_string().as_str() {
            "keyboard" => {
                let key_ord = dict
                    .get("key")
                    .and_then(|v| v.try_to::<i32>().ok())
                    .unwrap_or(0);
                InputMapping::Keyboard(Key::from_ord(key_ord))
            }
            "gamepad_button" => {
                let button_ord = dict
                    .get("button")
                    .and_then(|v| v.try_to::<i32>().ok())
                    .unwrap_or(0);
                let device = dict
                    .get("device")
                    .and_then(|v| v.try_to::<i32>().ok())
                    .unwrap_or(0);
                InputMapping::GamepadButton(JoyButton::from_ord(button_ord), device)
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
                let device = dict
                    .get("device")
                    .and_then(|v| v.try_to::<i32>().ok())
                    .unwrap_or(0);
                InputMapping::GamepadAxis(JoyAxis::from_ord(axis_ord), value, device)
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
/*// ── Imports ─────────────────────────────────────────────
use godot::classes::{Input, InputMap, InputEvent, InputEventKey,
    InputEventJoypadButton, InputEventJoypadMotion};
use godot::global::{JoyButton, JoyAxis};

// ── Enums (godot::global) ───────────────────────────────
JoyButton::A | B | X | Y | Back | Guide | Start
  | LeftStick | RightStick | LeftShoulder | RightShoulder
  | DpadUp | DpadDown | DpadLeft | DpadRight | Invalid // Invalid = -1 sentinel
JoyAxis::LeftX | LeftY | RightX | RightY
  | TriggerLeft | TriggerRight | Invalid

// ord round-trip (matches your Key serialization pattern):
JoyButton::try_from_ord(ord as i32); let ord = btn.ord();

// ── Polling (Input singleton) ───────────────────────────
let input = Input::singleton();
input.is_joy_button_pressed(device_id, JoyButton::A) -> bool;
input.get_joy_axis(device_id, JoyAxis::LeftX) -> f32;      // -1.0..1.0, NO deadzone
let pads = input.get_connected_joypads() -> PackedInt32Array; // device ids
input.is_joy_known(device_id) -> bool;                     // has standard mapping

// ── Hotplug signal (on Input singleton) ─────────────────
input.signals().joy_connection_changed()
    .connect_other(&this, |dev: i32, connected: bool| { /* ... */ });

// ── Rumble ──────────────────────────────────────────────
input.start_joy_vibration(device_id, weak_magnitude: f64,
                          strong_magnitude: f64, duration_secs: f64);
input.stop_joy_vibration(device_id);

// ── Event-side (_input / rebinding capture) ─────────────
fn input(&mut self, event: Gd<InputEvent>) {
    if let Some(k) = event.try_cast::<InputEventKey>().ok() {
        k.get_keycode();            // what you do now
    } else if let Some(b) = event.try_cast::<InputEventJoypadButton>().ok() {
        b.get_button_index();       // JoyButton
        b.get_device();             // i32, -1 = "all devices"
    } else if let Some(m) = event.try_cast::<InputEventJoypadMotion>().ok() {
        m.get_axis();               // JoyAxis
        m.get_axis_value();         // f32, sign = direction
    }
}

// Constructing events:
let mut ev = InputEventJoypadButton::new_gd();
ev.set_device(0);                   // omit/set -1 = any device
ev.set_button_index(JoyButton::A);
let mut mv = InputEventJoypadMotion::new_gd();
mv.set_axis(JoyAxis::LeftX);
mv.set_axis_value(1.0);

// ── Route B: runtime InputMap actions ───────────────────
let mut im = InputMap::singleton();
im.add_action("p1_jump");           // StringName, str literal ok
im.action_set_deadzone("p1_jump", 0.3);
im.action_add_event("p1_jump", &ev);          // stack multiple: key + pad
im.action_erase_events("p1_jump");            // before re-adding on rebind
im.has_action("p1_jump") -> bool;

// Polling actions (deadzone applied automatically):
input.is_action_pressed("p1_jump") -> bool;
input.is_action_just_pressed("p1_jump") -> bool;
input.is_action_released("p1_jump") -> bool;
input.get_action_strength("p1_jump") -> f32;  // analog 0..1

// ── Common idioms ───────────────────────────────────────
// digital threshold on raw axis:
let x = input.get_joy_axis(dev, JoyAxis::LeftX);
if x.abs() > 0.25 { /* moving at strength x */ }
// analog movement onto your z axis:
velocity.z += -x * speed_to_use;              // +X stick = right = -z in your code */
