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
