use godot::{
    classes::{IPanel, Input, Panel},
    global::Key,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct PauseMenuPanel {
    base: Base<Panel>,
    key_reset_trigger: bool,
}

#[godot_api]
impl IPanel for PauseMenuPanel {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            key_reset_trigger: false,
        }
    }
    fn process(&mut self, _delta: f32) {
        let input = Input::singleton();
        if input.is_key_label_pressed(Key::ESCAPE) && !self.key_reset_trigger {
            self.key_reset_trigger = true;
            if self.base().is_visible() {
                self.base_mut().set_visible(false);
            } else {
                self.base_mut().set_visible(true);
            }
        }
        if !input.is_key_label_pressed(Key::ESCAPE) {
            self.key_reset_trigger = false;
        }
    }
}

#[godot_api]
impl PauseMenuPanel {}
