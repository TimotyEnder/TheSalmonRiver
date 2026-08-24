use crate::game_scripts::player::Player;
use godot::{
    classes::{Area3D, INode3D, RigidBody3D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Fan {
    base: Base<Node3D>,
    wind_area: Option<Gd<Area3D>>,
    rigid_bodies: Vec<Gd<RigidBody3D>>,
    character_bodies: Vec<Gd<Player>>,
    push_force: f32,
}

#[godot_api]
impl INode3D for Fan {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            wind_area: None,
            rigid_bodies: Vec::new(),
            character_bodies: Vec::new(),
            push_force: 1.0,
        }
    }
    fn ready(&mut self) {
        self.wind_area = self
            .base()
            .find_child("WindArea")
            .and_then(|node| node.try_cast().ok());
        let this = self.to_gd();
        if let Some(ref mut wind) = self.wind_area {
            wind.signals()
                .area_entered()
                .connect_other(&this, Self::area_entered);
            wind.signals()
                .area_exited()
                .connect_other(&this, Self::area_exited);
        }
    }
    fn physics_process(&mut self, delta: f32) {
        let direction_vector = self.base().get_global_basis().col_a();

        self.character_bodies.retain(|cb| cb.is_instance_valid());
        self.rigid_bodies.retain(|rb| rb.is_instance_valid());

        self.character_bodies.iter_mut().for_each(|player| {
            let wind_delta = direction_vector * (self.push_force * 40.0 * delta);
            player.bind_mut().add_wind_force(wind_delta);
        });
        self.rigid_bodies.iter_mut().for_each(|rb| {
            let force = direction_vector * self.push_force;
            rb.apply_force(force * 10.0);
        });
    }
}

#[godot_api]
impl Fan {
    fn area_entered(&mut self, area: Gd<Area3D>) {
        if area.get_name().contains("Throwable") {
            godot_print!("Throwable entered fan");
            if let Some(rb) = area
                .get_parent()
                .and_then(|parent| parent.try_cast::<RigidBody3D>().ok())
            {
                godot_print!("Throwable added");
                self.rigid_bodies.push(rb);
            }
        } else if area.get_name().contains("Player") {
            godot_print!("Player entered fan");
            if let Some(player) = area
                .get_parent()
                .and_then(|parent| parent.try_cast::<Player>().ok())
            {
                godot_print!("Player added");
                self.character_bodies.push(player);
            }
        }
    }
    fn area_exited(&mut self, area: Gd<Area3D>) {
        if area.get_name().contains("Throwable") {
            godot_print!("Throwable exited fan");
            if let Some(rb) = area
                .get_parent()
                .and_then(|parent| parent.try_cast::<RigidBody3D>().ok())
            {
                if let Some(pos) = self.rigid_bodies.iter().position(|rbit| *rbit == rb) {
                    self.rigid_bodies.remove(pos);
                    godot_print!("Throwable removed");
                }
            }
        } else if area.get_name().contains("Player") {
            godot_print!("Player exited fan");
            if let Some(mut player) = area
                .get_parent()
                .and_then(|parent| parent.try_cast::<Player>().ok())
            {
                player.bind_mut().reset_wind();
                if let Some(pos) = self
                    .character_bodies
                    .iter()
                    .position(|cbbit| *cbbit == player)
                {
                    self.character_bodies.remove(pos);
                    godot_print!("Player removed");
                }
            }
        }
    }
}
