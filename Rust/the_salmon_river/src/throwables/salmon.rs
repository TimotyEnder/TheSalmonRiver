use crate::throwables::throwability::Throwability;

pub struct Salmon {}
impl Throwability for Salmon {
    fn deal_dmg(&mut self) -> u8 {
        1
    }

    fn visual_node_name(&self) -> &'static str {
        "Salmon"
    }

    fn use_ability(&mut self, player: &mut crate::player::Player) {
        player.health += 2;
    }
}
