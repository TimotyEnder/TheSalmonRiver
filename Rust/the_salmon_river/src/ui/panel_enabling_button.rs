use godot::{
    classes::{Button, IButton, Panel},
    prelude::*,
};

use crate::{
    game_managers::audio_manager::{AudioManager, AudioPlayBuilder},
    sound_utils::SoundEffect,
};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct PanelEnablingButton {
    base: Base<Button>,
    #[export]
    panel_to_enable: Option<Gd<Panel>>,
    #[export]
    disable_panel: bool,
    #[export]
    focus_after_click: Option<Gd<Button>>,
}

#[godot_api]
impl IButton for PanelEnablingButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            panel_to_enable: None,
            disable_panel: false,
            focus_after_click: None,
        }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        self.base()
            .signals()
            .button_up()
            .connect_other(&this, Self::on_press);
    }
}

#[godot_api]
impl PanelEnablingButton {
    #[func]
    fn on_press(&mut self) {
        let audio_manager = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");

        let position = self.base().get_global_position();
        if let Some(ref mut panel) = self.panel_to_enable
            && let Some(mut audio) = audio_manager
        {
            if self.disable_panel {
                audio.bind_mut().play_sound_built(
                    AudioPlayBuilder::play_sound_effect(SoundEffect::UICancel)
                        .at_position(position),
                );
                panel.set_visible(false);
                self.focus_on_panel();
            } else {
                audio.bind_mut().play_sound_built(
                    AudioPlayBuilder::play_sound_effect(SoundEffect::UIAccept)
                        .at_position(position),
                );
                panel.set_visible(true);
                self.focus_on_panel();
            }
        }
    }
    fn focus_on_panel(&mut self) {
        if let Some(ref mut focus_after_click) = self.focus_after_click {
            focus_after_click.grab_focus();
        }
    }
}
