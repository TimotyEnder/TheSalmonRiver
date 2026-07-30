use crate::throwables::throwability::Throwability;

pub struct Log {}
impl Throwability for Log {
    fn deal_dmg(&mut self) -> u8 {
        1
    }

    fn use_ability(&mut self) {
        // deploy it as a shild that eats one projectile or punch
    }
}
