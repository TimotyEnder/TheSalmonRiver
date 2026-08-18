use crate::{
    game_scripts::player::Player, sound_utils::SoundEffect, throwables::throwability::Throwability,
};

pub struct Salmon {}
impl Throwability for Salmon {
    fn deal_dmg(&mut self) -> u8 {
        1
    }

    fn visual_node_name(&self) -> &'static str {
        "Salmon"
    }

    fn sound_effect_on_break(&self) -> SoundEffect {
        SoundEffect::FishBreak
    }

    fn use_ability(&mut self, player: &mut Player) {
        player.heal(2);
        player.salmon_ability();
    }

    fn sound_effect_on_ability(&self) -> SoundEffect {
        SoundEffect::SalmonAbility
    }
}
