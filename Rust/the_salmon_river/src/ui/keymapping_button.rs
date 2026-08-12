use godot::{
    classes::{Button, IButton, InputEvent, InputEventKey},
    global::Key,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct KeyMappingButton {
    base: Base<Button>,
    is_listening: bool,
    key_saved: Option<Key>,
}

#[godot_api]
impl IButton for KeyMappingButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            is_listening: false,
            key_saved: None,
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
        self.key_saved = Some(key_code);
        godot_print!("Key recorded as saved: {:?}", key_code);
        self.is_listening = false;
        self.base_mut().set_text(&format!("{:?}", key_code));
        self.signals().on_key_changed().emit();
    }
    fn on_click(&mut self) {
        self.is_listening = true;
        self.base_mut().set_text("PRESS");
    }
    pub fn get_key_saved(&self) -> Key {
        if let Some(key) = self.key_saved {
            return key;
        } else {
            return Key::NONE;
        }
    }
    pub fn has_key_saved(&self) -> bool {
        return self.key_saved.is_some();
    }
}
