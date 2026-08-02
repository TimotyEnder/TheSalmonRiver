use godot::classes::class_macros::private::virtuals::Xrvrs::Gd;

use crate::player::Player;

pub trait Throwability {
    fn deal_dmg(&mut self) -> u8;
    fn use_ability(&mut self, player: Gd<Player>);
    fn visual_node_name(&self) -> &'static str;
}
