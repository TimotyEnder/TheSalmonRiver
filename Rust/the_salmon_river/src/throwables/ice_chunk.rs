use godot::classes::class_macros::private::virtuals::Xrvrs::Gd;

use crate::throwables::throwability::Throwability;

pub struct IceChunk {}
impl Throwability for IceChunk {
    fn deal_dmg(&mut self) -> u8 {
        2
    }

    fn visual_node_name(&self) -> &'static str {
        "IceChunk"
    }

    fn use_ability(&mut self, player: Gd<crate::player::Player>) {
        todo!()
    }
}
