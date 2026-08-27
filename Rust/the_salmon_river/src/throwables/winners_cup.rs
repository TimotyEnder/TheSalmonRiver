use crate::{sound_utils::SoundEffect::WinnerConfetti, throwables::throwability::Throwability};

pub struct WinnersCup {}
impl Throwability for WinnersCup {
    fn deal_dmg(&mut self) -> u8 {
        0
    }

    fn use_ability(&mut self, player: &mut crate::game_scripts::player::Player) {
        player.spawn_confetti();
    }

    fn visual_node_name(&self) -> &'static str {
        "WinnersCup"
    }

    fn sound_effect_on_break(&self) -> crate::sound_utils::SoundEffect {
        crate::sound_utils::SoundEffect::IceChunkBreak
    }

    fn sound_effect_on_ability(&self) -> crate::sound_utils::SoundEffect {
        WinnerConfetti
    }
}
