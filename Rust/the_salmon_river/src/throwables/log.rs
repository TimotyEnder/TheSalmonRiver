use crate::{
    game_scripts::player::Player, sound_utils::SoundEffect, throwables::throwability::Throwability,
};

pub struct Log {}
impl Throwability for Log {
    fn deal_dmg(&mut self) -> u8 {
        1
    }

    fn visual_node_name(&self) -> &'static str {
        "Log"
    }

    fn sound_effect_on_break(&self) -> SoundEffect {
        SoundEffect::LogBreak
    }

    fn use_ability(&mut self, player: &mut Player) {
        player.spawn_log_obsticle();
    }

    fn sound_effect_on_ability(&self) -> SoundEffect {
        SoundEffect::LogAbility
    }
}
