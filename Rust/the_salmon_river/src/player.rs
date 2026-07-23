use godot::classes::{
    AnimationPlayer, AnimationTree, CharacterBody3D, CollisionShape3D, ICharacterBody3D, Input,
    MeshInstance3D,
};
use godot::global::Key;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>,
    speed: f32,
    jump_force: f32,
    jumped: bool,
    ducked: bool,
    #[export]
    jump_key: Key,
    #[export]
    left_key: Key,
    #[export]
    right_key: Key,
    #[export]
    duck_key: Key,
    body_mesh: Option<Gd<Node3D>>,
    body_collider: Option<Gd<CollisionShape3D>>,
    lower_anim_tree: Option<Gd<AnimationTree>>,
    upper_anim_tree: Option<Gd<AnimationTree>>,
    facing_right: bool,
}
#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            body_collider: None,
            body_mesh: None,
            upper_anim_tree: None,
            lower_anim_tree: None,
            facing_right: true,
            speed: 2.0,
            jump_force: 5.0,
            jumped: false,
            ducked: false,
            jump_key: Key::W,
            left_key: Key::A,
            right_key: Key::D,
            duck_key: Key::S,
        }
    }
    fn ready(&mut self) {
        self.body_collider = self
            .base()
            .find_child("PlayerCollider")
            .and_then(|node| node.try_cast::<CollisionShape3D>().ok());
        self.body_mesh = self
            .base()
            .find_child("PlayerHead")
            .and_then(|node| node.try_cast::<Node3D>().ok());
        self.upper_anim_tree = self
            .base()
            .find_child("UpperAnim")
            .unwrap()
            .find_child("UpperAnimTree")
            .and_then(|node| node.try_cast::<AnimationTree>().ok());
        self.lower_anim_tree = self
            .base()
            .find_child("LowerAnim")
            .unwrap()
            .find_child("LowerAnimTree")
            .and_then(|node| node.try_cast::<AnimationTree>().ok());
    }
    fn physics_process(&mut self, delta: f64) {
        self.movement(delta);
        self.lower_animations();
    }
}
impl Player {
    fn movement(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut velocity = self.base().get_velocity();

        // Apply gravity
        velocity.y -= 20.0 * delta as f32;

        velocity.z = 0.0;
        if input.is_key_pressed(self.left_key) {
            velocity.z += self.speed;
            if self.facing_right {
                self.facing_right = false;
            }
        }
        if input.is_key_pressed(self.right_key) {
            velocity.z += -self.speed;
            if !self.facing_right {
                self.facing_right = true;
            }
        }
        if input.is_key_pressed(self.jump_key) && self.base().is_on_floor() && !self.jumped {
            self.jumped = true;
            velocity.y = self.jump_force;
        }
        if !input.is_key_pressed(self.jump_key) {
            self.jumped = false;
        }
        let duck_pressed = input.is_key_pressed(self.duck_key);
        let on_floor = self.base().is_on_floor();

        if let Some(ref mut collider) = self.body_collider
            && let Some(ref mut mesh) = self.body_mesh
        {
            let mut collider_scale = collider.get_scale();
            let mut mesh_scale = mesh.get_scale();
            if duck_pressed && on_floor {
                if !self.ducked {
                    self.ducked = true;
                    collider_scale.y *= 0.6;
                    mesh_scale.y *= 0.6;
                }
                velocity.z = 0.0;
            }
            if !duck_pressed && self.ducked {
                self.ducked = false;
                collider_scale.y /= 0.6;
                mesh_scale.y /= 0.6;
            }
            collider.set_scale(collider_scale);
            mesh.set_scale(mesh_scale);
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
    fn lower_animations(&mut self) {
        let grounded = self.base().is_on_floor();
        let side_velocity = self.base().get_velocity().z;
        if let Some(ref mut anim_tree) = self.lower_anim_tree {
            if !grounded {
                anim_tree.set("parameters/conditions/jump", &true.to_variant());
                anim_tree.set("parameters/conditions/idle", &false.to_variant());
                anim_tree.set("parameters/conditions/run", &false.to_variant());
                anim_tree.set("parameters/conditions/duck", &false.to_variant());
            } else {
                if self.ducked {
                    anim_tree.set("parameters/conditions/jump", &false.to_variant());
                    anim_tree.set("parameters/conditions/idle", &false.to_variant());
                    anim_tree.set("parameters/conditions/run", &false.to_variant());
                    anim_tree.set("parameters/conditions/duck", &true.to_variant());
                } else if side_velocity.abs() > 0.0 {
                    anim_tree.set("parameters/conditions/jump", &false.to_variant());
                    anim_tree.set("parameters/conditions/idle", &false.to_variant());
                    anim_tree.set("parameters/conditions/run", &true.to_variant());
                    anim_tree.set("parameters/conditions/duck", &false.to_variant());
                } else {
                    anim_tree.set("parameters/conditions/jump", &false.to_variant());
                    anim_tree.set("parameters/conditions/idle", &true.to_variant());
                    anim_tree.set("parameters/conditions/run", &false.to_variant());
                    anim_tree.set("parameters/conditions/duck", &false.to_variant());
                }
            }
        }
    }
    fn upper_animations(&mut self) {
        if let Some(ref mut anim_tree) = self.upper_anim_tree {}
    }
}
