use crate::{
    player::Player,
    throwables::{ice_chunk::IceChunk, salmon::Salmon, throwability::Throwability},
    utils::Direction,
};
use godot::{
    classes::{Area3D, CollisionShape3D, IRigidBody3D, RigidBody3D},
    prelude::*,
};
#[derive(GodotClass)]
#[class(base=RigidBody3D)]
pub struct Throwable {
    base: Base<RigidBody3D>,
    hitbox_area: Option<Gd<Area3D>>,
    throw_force: f32,
    in_hand: bool,
    thrown: bool,
    pub throwable_inner: Option<Box<dyn Throwability>>,
    thrower_id: Option<u8>,
}
#[godot_api]
impl IRigidBody3D for Throwable {
    fn init(base: Base<RigidBody3D>) -> Self {
        Self {
            base: base,
            hitbox_area: None,
            throw_force: 8.0,
            in_hand: false,
            thrown: false,
            throwable_inner: None,
            thrower_id: None,
        }
    }
    fn ready(&mut self) {
        self.become_throwable(Box::new(Salmon {}));
        self.base_mut().set_contact_monitor(true);
        self.base_mut().set_max_contacts_reported(1);
        self.ready_area();
        self.ready_collider();
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
    fn on_hitbox_entered(&mut self, area: Gd<Area3D>) {
        if area.get_name().contains("Grab") {
            let grab_player = area.get_parent().and_then(|hand| {
                hand.get_parent()
                    .and_then(|player| player.try_cast::<Player>().ok())
            });
            if let Some(mut player) = grab_player {
                player.bind_mut().pick_up_throwable();
                let this = self.to_gd();
                player
                    .signals()
                    .on_throwable_throw()
                    .connect_other(&this, Self::on_thrown);
                self.signals()
                    .throwable_grabbed()
                    .connect_other(&player, Player::on_throwable_grabbed);
                let player_script = player.bind();
                let player_num = player_script.get_player_num();
                self.thrower_id = Some(player_num);
                if !self.thrown {
                    let pickup_area_opt = player.find_child("RightHand").and_then(|rh| {
                        rh.find_child("PickUpArea")
                            .and_then(|pua| pua.try_cast::<Node3D>().ok())
                    });
                    if let Some(pickup_area) = pickup_area_opt {
                        self.base_mut().reparent(&pickup_area);
                        self.base_mut().set_position(Vector3::ZERO);
                        self.in_hand = true;
                    }
                    self.signals().throwable_grabbed().emit(player_num);
                }
            }
        }
    }
}
impl Throwable {
    pub fn become_throwable(&mut self, throwable: Box<dyn Throwability>) {
        self.base()
            .find_child(throwable.visual_node_name())
            .and_then(|f| f.try_cast::<Node3D>().ok())
            .and_then(|mut f| {
                f.set_visible(true);
                Some(())
            });
        self.throwable_inner = Some(throwable);
    }
    fn ready_area(&mut self) {
        self.hitbox_area = self
            .base()
            .find_child("ThrowableArea")
            .and_then(|g| g.try_cast::<Area3D>().ok());
        let this = self.to_gd();
        if let Some(ref mut grab) = self.hitbox_area {
            grab.signals()
                .area_entered()
                .connect_other(&this, Self::on_hitbox_entered);
        }
    }
    fn ready_collider(&mut self) {
        let this = self.to_gd();
        self.signals()
            .body_entered()
            .connect_other(&this, Self::on_physics_collision);
    }
    fn on_thrown(&mut self, dir: Direction) {
        self.thrown = true;
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
            y: self.throw_force / 4.0,
            z: {
                match dir {
                    Direction::Right => -self.throw_force,
                    Direction::Left => self.throw_force,
                }
            },
        };
        self.base_mut().apply_central_impulse(force_vector);
    }
    fn on_physics_collision(&mut self, body: Gd<Node>) {
        if self.thrown {
            self.base_mut().queue_free();
        }
    }
    pub fn does_player_hitstun(&self, player_num: u8) -> bool {
        if let Some(thrower) = self.thrower_id {
            return self.thrown && thrower != player_num;
        } else {
            false
        }
    }
    pub fn use_ability(&mut self, mut player: Gd<Player>) {
        if let Some(ti) = self.throwable_inner.as_mut() {
            ti.use_ability(player);
        }
        self.base_mut().queue_free();
    }
}
