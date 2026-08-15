use godot::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, GodotConvert)]
#[godot(via = i64)]
pub enum SoundEffect {
    PlayerDeathBlow = 0,
    PlayerPunch = 1,
    PlayerKnockDown = 2,
    UIAccept = 3,
    UICancel = 4,
    UIDecrement = 5,
    UIIncrement = 6,
}
impl SoundEffect {
    pub fn to_sound_effect_path(&self) -> &'static str {
        match self {
            SoundEffect::PlayerDeathBlow => "res://SoundAssets/death_blow.ogg",
            SoundEffect::PlayerPunch => "res://SoundAssets/punch.ogg",
            SoundEffect::PlayerKnockDown => "res://SoundAssets/knock_down.ogg",
            SoundEffect::UIAccept => "res://SoundAssets/ui_accept.ogg",
            SoundEffect::UICancel => "res://SoundAssets/ui_cancel.ogg",
            SoundEffect::UIDecrement => "res://SoundAssets/ui_decrement.ogg",
            SoundEffect::UIIncrement => "res://SoundAssets/ui_increment.ogg",
        }
    }
}
