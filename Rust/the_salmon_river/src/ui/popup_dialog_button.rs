use godot::{
    classes::{Button, Control, IButton},
    prelude::*,
};

use crate::{
    game_managers::audio_manager::{AudioManager, AudioPlayBuilder},
    sound_utils::SoundEffect::UIAccept,
};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct PopupDialogButton {
    base: Base<Button>,
    button_to_refocus: Option<Gd<Button>>,
    root_node: Option<Gd<Control>>,
}

#[godot_api]
impl IButton for PopupDialogButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            button_to_refocus: None,
            root_node: None,
        }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        self.base()
            .signals()
            .button_up()
            .connect_other(&this, Self::on_click);
        self.root_node = self.base().get_parent().and_then(|panel| {
            panel
                .get_parent()
                .and_then(|root| root.try_cast::<Control>().ok())
        });
    }
}

#[godot_api]
impl PopupDialogButton {
    fn on_click(&mut self) {
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(mut audio) = audio {
            audio.bind_mut().play_sound_built(
                AudioPlayBuilder::play_sound_effect(UIAccept)
                    .at_position(self.base().get_global_position()),
            );
        }
        if let Some(ref mut button) = self.button_to_refocus {
            button.grab_focus();
        }
        if let Some(ref mut root) = self.root_node {
            root.call_deferred("queue_free", &[]);
        }
    }
    pub fn assign_refocus_button(&mut self, button: Gd<Button>) {
        self.button_to_refocus = Some(button);
        self.base_mut().grab_focus();
    }
}
