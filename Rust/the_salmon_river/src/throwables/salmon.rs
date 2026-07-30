use crate::throwables::throwability::Throwability;

pub struct Salmon {}
impl Throwability for Salmon {
    fn deal_dmg(&mut self) -> u8 {
        1
    }

    fn use_ability(&mut self) {
        //heal player and make their next attack do double dmg and apply double hitstun
    }
}
