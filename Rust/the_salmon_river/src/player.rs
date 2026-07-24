use godot::classes::{
    AnimationTree, CharacterBody3D, CollisionShape3D, ICharacterBody3D, Input, Timer,
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
    #[export]
    punch_use_key: Key,
    #[export]
    throw_grab_ky: Key,
    body_mesh: Option<Gd<Node3D>>,
    body_collider: Option<Gd<CollisionShape3D>>,
    lower_anim_tree: Option<Gd<AnimationTree>>,
    upper_anim_tree: Option<Gd<AnimationTree>>,
    facing_right: bool,
    is_punching: bool,
    is_grab: bool,
    right_punch: bool,
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
            jump_force: 7.0,
            jumped: false,
            ducked: false,
            jump_key: Key::W,
            left_key: Key::A,
            right_key: Key::D,
            duck_key: Key::S,
            punch_use_key: Key::F,
            throw_grab_ky: Key::G,
            is_grab: false,
            is_punching: false,
            right_punch: false,
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
            .find_child("AnimationTree")
            .and_then(|node| node.try_cast::<AnimationTree>().ok());
        if let Some(ref mut anim_tree) = self.upper_anim_tree {
            anim_tree.set_active(true);
        }
        self.lower_anim_tree = self
            .base()
            .find_child("LowerAnim")
            .unwrap()
            .find_child("AnimationTree")
            .and_then(|node| node.try_cast::<AnimationTree>().ok());
    }
    fn physics_process(&mut self, delta: f64) {
        self.movement(delta);
        self.lower_animations();
        self.upper_animations();
        self.flip_based_on_facing_direction();
        self.action_process();
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
        if self.is_punching {
            return;
        }
        let grounded = self.base().is_on_floor();
        let side_velocity = self.base().get_velocity();
        if let Some(ref mut anim_tree) = self.upper_anim_tree {
            if side_velocity.z.abs() > 0.0 && grounded {
                anim_tree.set("parameters/conditions/jump", &false.to_variant());
                anim_tree.set("parameters/conditions/idle", &false.to_variant());
                anim_tree.set("parameters/conditions/run", &true.to_variant());
            } else {
                anim_tree.set("parameters/conditions/jump", &false.to_variant());
                anim_tree.set("parameters/conditions/idle", &true.to_variant());
                anim_tree.set("parameters/conditions/run", &false.to_variant());
            }
        }
    }
    fn flip_based_on_facing_direction(&mut self) {
        let mut scale = self.base().get_scale();
        if (self.facing_right && scale.z < 0.0 || !self.facing_right && scale.z > 0.0)
            && !self.is_punching
        {
            scale.z *= -1.0;
        }
        self.base_mut().set_scale(scale);
        if let Some(ref mut collider) = self.body_collider {
            let mut collider_scale = collider.get_scale();
            if collider_scale.z < 0.0 {
                collider_scale.z *= -1.0;
            }
            collider.set_scale(collider_scale);
        }
    }
    fn action_process(&mut self) {
        let input = Input::singleton();
        if input.is_key_pressed(self.punch_use_key) && !self.ducked {
            let this = self.to_gd();
            let _guard = self.base_mut();
            godot::task::spawn(Self::punch_routine(this));
        }
    }
    async fn punch_routine(mut this: Gd<Self>) {
        let original_speed;
        let mut timer;
        {
            let mut bind = this.bind_mut();
            if bind.is_punching || bind.is_grab {
                return;
            }

            bind.is_punching = true;
            original_speed = bind.speed;
            bind.speed *= 0.3;
            let mut right_punch = bind.right_punch;
            if let Some(ref mut upper_anim) = bind.upper_anim_tree {
                if right_punch {
                    upper_anim.set("parameters/conditions/r_punch", &true.to_variant());
                    right_punch = false;
                } else if !right_punch {
                    upper_anim.set("parameters/conditions/l_punch", &true.to_variant());
                    right_punch = true;
                }
            }
            bind.right_punch = right_punch;
            let mut t = Timer::new_alloc();
            t.set_wait_time(0.25);
            t.set_one_shot(true);
            timer = t.clone();
            bind.base_mut().add_child(&t.upcast::<Node>());
        }

        timer.start();
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;

        {
            let mut bind = this.bind_mut();
            bind.is_punching = false;
            bind.speed = original_speed;
            if let Some(ref mut upper_anim) = bind.upper_anim_tree {
                upper_anim.set("parameters/conditions/l_punch", &false.to_variant());
                upper_anim.set("parameters/conditions/r_punch", &false.to_variant());
            }
        }
        timer.upcast::<Node>().queue_free();
    }
}
