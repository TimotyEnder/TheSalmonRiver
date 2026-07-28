use crate::{player::Player, utils::Direction};
use godot::{
    classes::{Area3D, IRigidBody3D, RigidBody3D},
    prelude::*,
};
#[derive(GodotClass)]
#[class(base=RigidBody3D)]
pub struct Throwable {
    base: Base<RigidBody3D>,
    grab_area: Option<Gd<Area3D>>,
    throw_force: f32,
    in_hand: bool,
}
#[godot_api]
impl IRigidBody3D for Throwable {
    fn init(base: Base<RigidBody3D>) -> Self {
        Self {
            base: base,
            grab_area: None,
            throw_force: 8.0,
            in_hand: false,
        }
    }
    fn ready(&mut self) {
        self.ready_grab_area();
    }
    fn process(&mut self, delta: f64) {
        if self.in_hand {
            self.base_mut().set_scale(Vector3::ONE);
            self.base_mut().set_rotation(Vector3::ZERO);
        }
    }
}
#[godot_api]
impl Throwable {
    #[signal]
    fn throwable_grabbed(player_num: u8);
    #[func]
    fn on_grab(&mut self, area: Gd<Area3D>) {
        if area.get_name().contains("Grab") {
            let grab_player = area.get_parent().and_then(|hand| {
                hand.get_parent()
                    .and_then(|player| player.try_cast::<Player>().ok())
            });
            if let Some(mut player) = grab_player {
                player.bind_mut().pick_up();
                let this = self.to_gd();
                player
                    .signals()
                    .on_throwable_throw()
                    .connect_other(&this, Self::on_thrown);
                let player_script = player.bind();
                let player_num = player_script.get_player_num();
                self.signals().throwable_grabbed().emit(player_num);
                let pickup_area_opt = player.find_child("RightHand").and_then(|rh| {
                    rh.find_child("PickUpArea")
                        .and_then(|pua| pua.try_cast::<Node3D>().ok())
                });
                if let Some(pickup_area) = pickup_area_opt {
                    self.base_mut().reparent(&pickup_area);
                    self.base_mut().set_position(Vector3::ZERO);
                    self.in_hand = true;
                }
            }
        }
    }
}
impl Throwable {
    fn ready_grab_area(&mut self) {
        self.grab_area = self
            .base()
            .find_child("GrabArea")
            .and_then(|g| g.try_cast::<Area3D>().ok());
        let this = self.to_gd();
        if let Some(ref mut grab) = self.grab_area {
            grab.signals()
                .area_entered()
                .connect_other(&this, Self::on_grab);
        }
    }
    fn on_thrown(&mut self, dir: Direction) {
        let scene_root_opt = self.base().get_tree().get_current_scene();
        if let Some(scene_root) = scene_root_opt {
            self.base_mut()
                .call_deferred("reparent", &[scene_root.to_variant()]);
        }
        self.in_hand = false;
        self.base_mut().set_linear_velocity(Vector3::ZERO);
        self.base_mut().set_rotation(Vector3::ZERO);
        self.base_mut().set_scale(Vector3::ONE);
        let force_vector = Vector3 {
            x: 0.0,
            y: self.throw_force / 3.0,
            z: {
                match dir {
                    Direction::Right => -self.throw_force,
                    Direction::Left => self.throw_force,
                }
            },
        };
        self.base_mut().apply_central_impulse(force_vector);
    }
}
