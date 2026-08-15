use godot::{
    classes::{Button, IButton, InputEvent, InputEventKey},
    global::Key,
    prelude::*,
};

use crate::ui::player_controls_panel::PlayerControlsPanel;

#[derive(GodotClass)]
#[class(base=Button)]
pub struct KeyMappingButton {
    base: Base<Button>,
    is_listening: bool,
    key_saved: Option<Key>,
    parent_panel: Option<Gd<PlayerControlsPanel>>,
}

#[godot_api]
impl IButton for KeyMappingButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            is_listening: false,
            key_saved: None,
            parent_panel: None,
        }
    }
    fn ready(&mut self) {
        self.base_mut().set_text("ASSIGN");
        let this = self.to_gd();
        self.base()
            .signals()
            .pressed()
            .connect_other(&this, Self::on_click);
    }
    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(key_event) = event.try_cast::<InputEventKey>().ok()
            && self.is_listening
        {
            let key_code = key_event.get_keycode();
            self.save_key(key_code);
        }
    }
}

#[godot_api]
impl KeyMappingButton {
    #[signal]
    pub fn on_key_changed();
    pub fn save_key(&mut self, key_code: Key) {
        if key_code != Key::ESCAPE {
            self.key_saved = Some(key_code);
        }
        self.is_listening = false;
        self.set_label_to_keycode();
    }
    fn set_label_to_keycode(&mut self) {
        if let Some(key_code) = self.key_saved {
            self.base_mut().set_text(&format!("{:?}", key_code));
            self.signals().on_key_changed().emit();
        }
    }

    fn on_click(&mut self) {
        let id = self.base().instance_id();
        self.is_listening = true;
        self.base_mut().set_text("PRESS");
        if let Some(ref mut parent) = self.parent_panel {
            parent.bind_mut().exclusivety_reset(id);
        }
    }
    pub fn get_key_saved(&self) -> Key {
        if let Some(key) = self.key_saved {
            return key;
        } else {
            return Key::NONE;
        }
    }
    pub fn has_key_saved(&self) -> bool {
        if let Some(key) = self.key_saved {
            return key != Key::NONE;
        } else {
            return false;
        }
    }
    pub fn exclusivity_reset(&mut self) {
        self.is_listening = false;
        self.set_label_to_keycode();
    }
    pub fn set_parent_control_panel(&mut self, panel: Gd<PlayerControlsPanel>) {
        self.parent_panel = Some(panel);
    }
}
