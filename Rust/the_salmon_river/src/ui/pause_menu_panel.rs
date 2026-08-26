use godot::{
    classes::{Button, IPanel, Input, Panel},
    global::{JoyButton, Key},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct PauseMenuPanel {
    base: Base<Panel>,
    key_reset_trigger: bool,
    #[export]
    focus_button: Option<Gd<Button>>,
}

#[godot_api]
impl IPanel for PauseMenuPanel {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            key_reset_trigger: false,
            focus_button: None,
        }
    }
    fn process(&mut self, _delta: f32) {
        let input = Input::singleton();
        if (input.is_key_label_pressed(Key::ESCAPE)
            || input.is_joy_button_pressed(-1, JoyButton::START))
            && !self.key_reset_trigger
        {
            self.key_reset_trigger = true;
            if self.base().is_visible() {
                self.base_mut().set_visible(false);
            } else {
                self.base_mut().set_visible(true);
            }
            if let Some(ref mut button) = self.focus_button {
                button.grab_focus();
            }
        }
        if !input.is_key_label_pressed(Key::ESCAPE)
            || input.is_joy_button_pressed(-1, JoyButton::START)
        {
            self.key_reset_trigger = false;
        }
    }
}

#[godot_api]
impl PauseMenuPanel {}
