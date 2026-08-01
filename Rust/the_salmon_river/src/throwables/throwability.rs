pub trait Throwability {
    fn deal_dmg(&mut self) -> u8;
    fn use_ability(&mut self);
    fn visual_node_name(&self) -> &'static str;
}
