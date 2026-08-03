use crate::throwables::throwability::Throwability;

pub struct Log {}
impl Throwability for Log {
    fn deal_dmg(&mut self) -> u8 {
        1
    }

    fn visual_node_name(&self) -> &'static str {
        "Log"
    }

    fn use_ability(&mut self, player: &mut crate::player::Player) {
        player.spawn_log_obsticle();
    }
}
