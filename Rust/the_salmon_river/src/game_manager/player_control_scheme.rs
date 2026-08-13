use godot::classes::IRefCounted;
use godot::global::Key;
use godot::prelude::*;
#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct PlayerControlScheme {
    #[var]
    pub jump_key: Key,
    #[var]
    pub left_key: Key,
    #[var]
    pub right_key: Key,
    #[var]
    pub duck_key: Key,
    #[var]
    pub punch_use_key: Key,
    #[var]
    pub grab_throw_key: Key,
}
#[godot_api]
impl IRefCounted for PlayerControlScheme {
    fn init(_base: Base<RefCounted>) -> Self {
        Self {
            jump_key: Key::NONE,
            left_key: Key::NONE,
            right_key: Key::NONE,
            duck_key: Key::NONE,
            punch_use_key: Key::NONE,
            grab_throw_key: Key::NONE,
        }
    }
}
impl Default for PlayerControlScheme {
    fn default() -> Self {
        Self {
            jump_key: Key::NONE,
            left_key: Key::NONE,
            right_key: Key::NONE,
            duck_key: Key::NONE,
            punch_use_key: Key::NONE,
            grab_throw_key: Key::NONE,
        }
    }
}
impl PlayerControlScheme {
    pub fn to_dict(&self) -> Dictionary<StringName, Variant> {
        let mut dict: Dictionary<StringName, Variant> = Dictionary::new();
        dict.set("jump_key", self.jump_key);
        dict.set("left_key", self.left_key);
        dict.set("right_key", self.right_key);
        dict.set("duck_key", self.duck_key);
        dict.set("punch_use_key", self.punch_use_key);
        dict.set("grab_throw_key", self.grab_throw_key);
        dict
    }
    pub fn from_dict(dict: &Dictionary<Variant, Variant>) -> Self {
        Self {
            jump_key: dict
                .get(&GString::from("jump_key"))
                .map(Self::key_from_variant)
                .unwrap_or(Key::NONE),
            left_key: dict
                .get(&GString::from("left_key"))
                .map(Self::key_from_variant)
                .unwrap_or(Key::NONE),
            right_key: dict
                .get(&GString::from("right_key"))
                .map(Self::key_from_variant)
                .unwrap_or(Key::NONE),
            duck_key: dict
                .get(&GString::from("duck_key"))
                .map(Self::key_from_variant)
                .unwrap_or(Key::NONE),
            punch_use_key: dict
                .get(&GString::from("punch_use_key"))
                .map(Self::key_from_variant)
                .unwrap_or(Key::NONE),
            grab_throw_key: dict
                .get(&GString::from("grab_throw_key"))
                .map(Self::key_from_variant)
                .unwrap_or(Key::NONE),
        }
    }
    fn key_from_variant(value: Variant) -> Key {
        let ord = value
            .try_to::<i64>()
            .or_else(|_| value.try_to::<f64>().map(|f| f as i64))
            .unwrap_or(0);
        Key::try_from_ord(ord as i32).unwrap_or(Key::NONE)
    }
}
