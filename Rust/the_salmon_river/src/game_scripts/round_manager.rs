use godot::{classes::INode3D, prelude::*};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct RoundManager {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for RoundManager {
    fn init(base: Base<Node3D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {}
}

#[godot_api]
impl RoundManager {}
