use crate::throwables::throwability::Throwability;

pub struct IceChunk {}
impl Throwability for IceChunk {
    fn deal_dmg(&mut self) -> u8 {
        2
    }

    fn use_ability(&mut self) {
        //implement a dash
    }
}
