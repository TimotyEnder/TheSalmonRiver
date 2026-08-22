use godot::{
    classes::{Button, Control, IControl, Sprite2D},
    prelude::*,
};

use crate::{
    game_managers::audio_manager::{AudioManager, AudioPlayBuilder},
    sound_utils::SoundEffect::UIAccept,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct CharacterSelector {
    base: Base<Control>,
    current_character_index: usize,
    characters: Vec<Gd<Sprite2D>>,
    values: Vec<StringName>,
}

#[godot_api]
impl IControl for CharacterSelector {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            current_character_index: 0,
            characters: Vec::new(),
            values: Vec::new(),
        }
    }
    fn ready(&mut self) {
        let characters_node = self
            .base()
            .find_child("Characters")
            .and_then(|c| c.try_cast::<Control>().ok());
        if let Some(characters) = characters_node {
            characters.get_children().iter_shared().for_each(|node| {
                if let Some(control) = node.try_cast::<Sprite2D>().ok() {
                    self.values.push(control.get_name());
                    self.characters.push(control);
                }
            });
        }
        self.update_selector();
        let this = self.to_gd();
        if let Some(left) = self
            .base()
            .find_child("LeftButton")
            .and_then(|but| but.try_cast::<Button>().ok())
            && let Some(right) = self
                .base()
                .find_child("RightButton")
                .and_then(|but| but.try_cast::<Button>().ok())
        {
            right
                .signals()
                .button_up()
                .connect_other(&this, Self::right);
            left.signals().button_up().connect_other(&this, Self::left);
        }
    }
}

#[godot_api]
impl CharacterSelector {
    #[signal]
    pub fn value_changed();
    fn update_selector(&mut self) {
        self.characters
            .iter_mut()
            .for_each(|char| char.set_visible(false));
        self.characters[self.current_character_index].set_visible(true);
    }
    pub fn get_value(&self) -> String {
        let value = self.values[self.current_character_index].clone();
        godot_print!("{value}");
        self.values[self.current_character_index]
            .clone()
            .to_string()
    }
    fn left(&mut self) {
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(mut audio) = audio {
            audio.bind_mut().play_sound_built(
                AudioPlayBuilder::play_sound_effect(UIAccept)
                    .at_position(self.base().get_global_position()),
            );
        }
        self.current_character_index =
            (self.current_character_index + self.characters.len() - 1) % self.characters.len();
        self.update_selector();
        self.signals().value_changed().emit();
    }
    fn right(&mut self) {
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(mut audio) = audio {
            audio.bind_mut().play_sound_built(
                AudioPlayBuilder::play_sound_effect(UIAccept)
                    .at_position(self.base().get_global_position()),
            );
        }
        self.current_character_index =
            (self.current_character_index + self.characters.len() + 1) % self.characters.len();
        self.update_selector();
        self.signals().value_changed().emit();
    }
}
