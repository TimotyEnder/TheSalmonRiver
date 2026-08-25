use godot::{global::Key, prelude::*};

use crate::game_managers::input_mapping::InputMapping;
#[derive(Clone, Copy)]
pub struct PlayerControlScheme {
    pub jump_key: InputMapping,
    pub left_key: InputMapping,
    pub right_key: InputMapping,
    pub duck_key: InputMapping,
    pub punch_use_key: InputMapping,
    pub grab_throw_key: InputMapping,
}

impl Default for PlayerControlScheme {
    fn default() -> Self {
        Self {
            jump_key: InputMapping::Keyboard(Key::NONE),
            left_key: InputMapping::Keyboard(Key::NONE),
            right_key: InputMapping::Keyboard(Key::NONE),
            duck_key: InputMapping::Keyboard(Key::NONE),
            punch_use_key: InputMapping::Keyboard(Key::NONE),
            grab_throw_key: InputMapping::Keyboard(Key::NONE),
        }
    }
}
impl PlayerControlScheme {
    pub fn to_dict(&self) -> Dictionary<StringName, Variant> {
        let mut dict: Dictionary<StringName, Variant> = Dictionary::new();
        dict.set("jump_key", &self.jump_key.to_dict().to_variant());
        dict.set("left_key", &self.left_key.to_dict().to_variant());
        dict.set("right_key", &self.right_key.to_dict().to_variant());
        dict.set("duck_key", &self.duck_key.to_dict().to_variant());
        dict.set("punch_use_key", &self.punch_use_key.to_dict().to_variant());
        dict.set(
            "grab_throw_key",
            &self.grab_throw_key.to_dict().to_variant(),
        );
        dict
    }
    pub fn from_dict(dict: &Dictionary<Variant, Variant>) -> Self {
        Self {
            jump_key: dict
                .get(&GString::from("jump_key"))
                .map(|variant| {
                    Self::input_mapping_from_variant_dict(
                        variant
                            .try_to::<Dictionary<Variant, Variant>>()
                            .unwrap_or_default(),
                    )
                })
                .unwrap_or(InputMapping::Keyboard(Key::NONE)),
            left_key: dict
                .get(&GString::from("left_key"))
                .map(|variant| {
                    Self::input_mapping_from_variant_dict(
                        variant
                            .try_to::<Dictionary<Variant, Variant>>()
                            .unwrap_or_default(),
                    )
                })
                .unwrap_or(InputMapping::Keyboard(Key::NONE)),
            right_key: dict
                .get(&GString::from("right_key"))
                .map(|variant| {
                    Self::input_mapping_from_variant_dict(
                        variant
                            .try_to::<Dictionary<Variant, Variant>>()
                            .unwrap_or_default(),
                    )
                })
                .unwrap_or(InputMapping::Keyboard(Key::NONE)),
            duck_key: dict
                .get(&GString::from("duck_key"))
                .map(|variant| {
                    Self::input_mapping_from_variant_dict(
                        variant
                            .try_to::<Dictionary<Variant, Variant>>()
                            .unwrap_or_default(),
                    )
                })
                .unwrap_or(InputMapping::Keyboard(Key::NONE)),
            punch_use_key: dict
                .get(&GString::from("punch_use_key"))
                .map(|variant| {
                    Self::input_mapping_from_variant_dict(
                        variant
                            .try_to::<Dictionary<Variant, Variant>>()
                            .unwrap_or_default(),
                    )
                })
                .unwrap_or(InputMapping::Keyboard(Key::NONE)),
            grab_throw_key: dict
                .get(&GString::from("grab_throw_key"))
                .map(|variant| {
                    Self::input_mapping_from_variant_dict(
                        variant
                            .try_to::<Dictionary<Variant, Variant>>()
                            .unwrap_or_default(),
                    )
                })
                .unwrap_or(InputMapping::Keyboard(Key::NONE)),
        }
    }
    fn input_mapping_from_variant_dict(value: Dictionary<Variant, Variant>) -> InputMapping {
        InputMapping::from_dict(value)
    }
}
