use godot::{classes::INode3D, prelude::*};

use crate::game_scripts::{flying_duck::FlyingDuck, saw_blade::SawBlade};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct SawPlatform {
    base: Base<Node3D>,
    #[export]
    duck: Option<Gd<FlyingDuck>>,
    #[export]
    saw: Option<Gd<SawBlade>>,
}

#[godot_api]
impl INode3D for SawPlatform {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            duck: None,
            saw: None,
        }
    }
}

#[godot_api]
impl SawPlatform {
    #[func]
    pub fn unmute(&mut self) {
        if let Some(ref mut duck) = self.duck
            && let Some(ref mut saw) = self.saw
        {
            duck.bind_mut().set_play_flap(true);
            saw.bind_mut().play_sound();
        }
    }
    #[func]
    pub fn mute(&mut self) {
        if let Some(ref mut duck) = self.duck
            && let Some(ref mut saw) = self.saw
        {
            duck.bind_mut().set_play_flap(false);
            saw.bind_mut().stop_sound();
        }
    }
}
