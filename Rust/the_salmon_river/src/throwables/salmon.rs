use godot::classes::class_macros::private::virtuals::Xrvrs::Gd;

use crate::throwables::throwability::Throwability;

pub struct Salmon {}
impl Throwability for Salmon {
    fn deal_dmg(&mut self) -> u8 {
        1
    }

    fn visual_node_name(&self) -> &'static str {
        "Salmon"
    }

    fn use_ability(&mut self, mut player: Gd<crate::player::Player>) {
        player.bind_mut().health += 2;
    }
}
