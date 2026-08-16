use crate::{game_scripts::player::Player, sound_utils::SoundEffect};

pub trait Throwability {
    fn deal_dmg(&mut self) -> u8;
    fn use_ability(&mut self, player: &mut Player);
    fn visual_node_name(&self) -> &'static str;
    fn sound_effect(&self) -> SoundEffect;
}
