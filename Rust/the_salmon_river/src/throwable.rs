use godot::{
    classes::{Area3D, IRigidBody3D, RigidBody3D},
    prelude::*,
};
#[derive(GodotClass)]
#[class(base=RigidBody3D)]
pub struct Throwable {
    base: Base<RigidBody3D>,
    grab_area: Option<Gd<Area3D>>,
}
#[godot_api]
impl IRigidBody3D for Throwable {
    fn init(base: Base<RigidBody3D>) -> Self {
        Self {
            base: base,
            grab_area: None,
        }
    }
    fn ready(&mut self) {}
    fn process(&mut self, delta: f64) {}
}
impl Throwable {
    fn ready_hitbox(&mut self) {
        // self.hitbox = self
        //     .base()
        //     .find_child("PlayerHitBox")
        //     .and_then(|p| p.try_cast::<Area3D>().ok());
        // let on_hit_callable = self.base().callable("on_player_hit");
        // if let Some(ref mut hitbox) = self.hitbox {
        //     hitbox.connect("area_entered", &on_hit_callable);
        // }
    }
}
