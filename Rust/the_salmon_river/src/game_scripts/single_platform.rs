use godot::{
    classes::{Area3D, INode3D},
    prelude::*,
};

use crate::game_scripts::{flying_duck::FlyingDuck, player::Player};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct SinglePlatform {
    base: Base<Node3D>,
    one_way_area: Option<Gd<Area3D>>,
    collider_enabled: bool,
    #[export]
    duck1: Option<Gd<FlyingDuck>>,
    #[export]
    duck2: Option<Gd<FlyingDuck>>,
}

#[godot_api]
impl INode3D for SinglePlatform {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            collider_enabled: true,
            one_way_area: None,
            duck1: None,
            duck2: None,
        }
    }
    fn ready(&mut self) {
        self.one_way_area = self
            .base()
            .find_child("OneWayEnsureArea")
            .and_then(|owea| owea.try_cast::<Area3D>().ok());
        let this = self.to_gd();
        if let Some(ref mut area) = self.one_way_area {
            area.signals()
                .area_entered()
                .connect_other(&this, Self::on_oneway_area_enter);
            area.signals()
                .area_exited()
                .connect_other(&this, Self::on_oneway_area_exit);
        }
    }
}

#[godot_api]
impl SinglePlatform {
    #[func]
    pub fn enable_collider(&mut self) {
        self.collider_enabled = true;
    }
    #[func]
    pub fn disable_collider(&mut self) {
        self.collider_enabled = false;
    }
    #[func]
    pub fn unmute(&mut self) {
        if let Some(ref mut duck1) = self.duck1 {
            duck1.bind_mut().set_play_flap(true);
        }
        if let Some(ref mut duck2) = self.duck2 {
            duck2.bind_mut().set_play_flap(true);
        }
    }
    #[func]
    pub fn mute(&mut self) {
        if let Some(ref mut duck1) = self.duck1 {
            duck1.bind_mut().set_play_flap(false);
        }
        if let Some(ref mut duck2) = self.duck2 {
            duck2.bind_mut().set_play_flap(false);
        }
    }
    pub fn on_oneway_area_enter(&mut self, area: Gd<Area3D>) {
        let debug = self.collider_enabled;
        godot_print!("{debug}");
        if self.base().get_global_position().y < area.get_global_position().y
            && self.collider_enabled
        {
            if let Some(mut player) = area
                .get_parent()
                .and_then(|player| player.try_cast::<Player>().ok())
            {
                player
                    .bind_mut()
                    .place_on_one_way_platform(self.base().get_global_position());
            }
        }
    }
    pub fn on_oneway_area_exit(&mut self, area: Gd<Area3D>) {
        if let Some(mut player) = area
            .get_parent()
            .and_then(|player| player.try_cast::<Player>().ok())
        {
            player.bind_mut().displace_from_one_way_platform();
        }
    }
}
