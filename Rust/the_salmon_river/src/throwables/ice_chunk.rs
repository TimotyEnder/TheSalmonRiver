use godot::obj::WithBaseField;

use crate::{
    game_scripts::player::Player, sound_utils::SoundEffect, throwables::throwability::Throwability,
};

pub struct IceChunk {}
impl Throwability for IceChunk {
    fn deal_dmg(&mut self) -> u8 {
        2
    }

    fn visual_node_name(&self) -> &'static str {
        "IceChunk"
    }

    fn sound_effect_on_break(&self) -> SoundEffect {
        SoundEffect::IceChunkBreak
    }

    fn use_ability(&mut self, player: &mut Player) {
        let this = player.to_gd();
        let _guard = player.base_mut();
        godot::task::spawn(Player::ice_chunk_dash_routine(this));
    }

    fn sound_effect_on_ability(&self) -> SoundEffect {
        SoundEffect::IceChunkAbility
    }
}
