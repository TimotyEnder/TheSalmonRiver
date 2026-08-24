use godot::{classes::INode3D, prelude::*};

use crate::game_scripts::fan::Fan;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct FanPlatform {
    base: Base<Node3D>,
    #[export]
    fan: Option<Gd<Fan>>,
}

#[godot_api]
impl INode3D for FanPlatform {
    fn init(base: Base<Node3D>) -> Self {
        Self { base, fan: None }
    }
    fn ready(&mut self) {}
}

#[godot_api]
impl FanPlatform {
    #[func]
    pub fn unmute(&mut self) {
        if let Some(ref mut fan) = self.fan {
            fan.bind_mut().play_sound();
        }
    }
    #[func]
    pub fn mute(&mut self) {
        if let Some(ref mut fan) = self.fan {
            fan.bind_mut().stop_sound();
        }
    }
}
