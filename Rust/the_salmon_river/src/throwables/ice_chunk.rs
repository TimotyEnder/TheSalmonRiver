use crate::throwables::throwability::Throwability;

pub struct IceChunk {}
impl Throwability for IceChunk {
    fn deal_dmg(&mut self) -> u8 {
        2
    }

    fn visual_node_name(&self) -> &'static str {
        "IceChunk"
    }

    fn use_ability(&mut self, _player: &mut crate::player::Player) {
        todo!()
    }
}
