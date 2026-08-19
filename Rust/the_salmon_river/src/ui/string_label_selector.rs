use godot::{
    classes::{Button, Control, IControl, RichTextLabel},
    prelude::*,
};

use crate::{game_managers::audio_manager::AudioManager, sound_utils::SoundEffect};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct StringLabelSelector {
    base: Base<Control>,
    right_button: Option<Gd<Button>>,
    left_button: Option<Gd<Button>>,
    label: Option<Gd<RichTextLabel>>,
    #[export]
    possible_values: PackedArray<GString>,
    current_selection_pos: usize,
}

#[godot_api]
impl IControl for StringLabelSelector {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            right_button: None,
            left_button: None,
            label: None,
            possible_values: PackedArray::new(),
            current_selection_pos: 0,
        }
    }
    fn ready(&mut self) {
        self.left_button = self
            .base()
            .find_child("LeftButton")
            .and_then(|button| button.try_cast::<Button>().ok());
        self.right_button = self
            .base()
            .find_child("RightButton")
            .and_then(|button| button.try_cast::<Button>().ok());
        self.label = self
            .base()
            .find_child("Label")
            .and_then(|button| button.try_cast::<RichTextLabel>().ok());
        let this = self.to_gd();
        if let Some(ref mut label) = self.label {
            label.set_text(&format!(
                "{}",
                self.possible_values.get(0).unwrap_or(GString::new())
            ));
        }
        if let Some(ref mut inc) = self.right_button
            && let Some(ref mut dec) = self.left_button
        {
            inc.signals()
                .button_up()
                .connect_other(&this, Self::on_right);
            dec.signals()
                .button_up()
                .connect_other(&this, Self::on_left);
        }
    }
}

#[godot_api]
impl StringLabelSelector {
    pub fn load_value(&mut self, value: GString) -> bool {
        let index_found_opt = self.possible_values.find(&value, None);
        if let Some(index_found) = index_found_opt {
            self.current_selection_pos = index_found;
            self.update_label();
            return true;
        }
        return false;
    }
    #[signal]
    pub fn value_changed();
    fn update_label(&mut self) {
        if let Some(ref mut label) = self.label {
            label.set_text(&format!(
                "{}",
                self.possible_values
                    .get(self.current_selection_pos)
                    .unwrap_or(GString::new())
            ));
        }
    }
    fn on_right(&mut self) {
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(mut audio) = audio {
            let len = self.possible_values.len();
            if len == 0 {
                return;
            }
            self.current_selection_pos = (self.current_selection_pos + 1) % len;
            audio
                .bind_mut()
                .play_sound(SoundEffect::UIIncrement, self.base().get_global_position());
            self.update_label();
            self.signals().value_changed().emit();
        }
    }
    fn on_left(&mut self) {
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(mut audio) = audio {
            let len = self.possible_values.len();
            if len == 0 {
                return;
            }
            self.current_selection_pos = (self.current_selection_pos + len - 1) % len;
            audio
                .bind_mut()
                .play_sound(SoundEffect::UIDecrement, self.base().get_global_position());
            self.update_label();
            self.signals().value_changed().emit();
        }
    }
    pub fn get_value(&self) -> GString {
        self.possible_values
            .get(self.current_selection_pos)
            .unwrap_or(GString::new())
    }
}
