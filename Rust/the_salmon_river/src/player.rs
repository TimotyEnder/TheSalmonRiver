use godot::classes::{
    AnimationTree, Area3D, CharacterBody3D, CollisionShape3D, ICharacterBody3D, Input, Label3D,
    Timer,
};
use godot::global::Key;
use godot::prelude::*;

use crate::utils;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>,
    speed: f32,
    jump_force: f32,
    punch_force: f32,
    jumped: bool,
    ducked: bool,
    hit_stun: bool,
    knock_back_force: f32,
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
    hitbox: Option<Gd<Area3D>>,
    lower_anim_tree: Option<Gd<AnimationTree>>,
    upper_anim_tree: Option<Gd<AnimationTree>>,
    #[export]
    facing_right: bool,
    is_punching: bool,
    is_grab: bool,
    right_punch: bool,
    #[export]
    player_num: u8,
    player_label: Option<Gd<Label3D>>,
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
            hitbox: None,
            facing_right: true,
            speed: 2.0,
            jump_force: 7.0,
            punch_force: 0.5,
            knock_back_force: 6.0,
            jumped: false,
            ducked: false,
            hit_stun: false,
            jump_key: Key::W,
            left_key: Key::A,
            right_key: Key::D,
            duck_key: Key::S,
            punch_use_key: Key::F,
            throw_grab_ky: Key::G,
            is_grab: false,
            is_punching: false,
            right_punch: false,
            player_num: 1,
            player_label: None,
        }
    }
    fn ready(&mut self) {
        self.ready_body();
        self.ready_animations();
        self.ready_hitbox();
        self.ready_label();
        self.ready_groups();
    }
    fn process(&mut self, delta: f64) {
        if !self.hit_stun {
            self.movement(delta);
            self.lower_animations();
            self.upper_animations();
            self.flip_based_on_facing_direction();
            self.action_process();
        }
    }
}
#[godot_api]
impl Player {
    #[func]
    fn on_player_hit(&mut self, area: Gd<Area3D>) {
        let area_name = area.get_name();
        let area_groups = area.get_groups();
        if !area_groups.contains(&format!("p{}", self.player_num)) {
            godot_print!("{}", area_name);
            if area_name.contains("Grab") {
                godot_print!("Grab");
            } else if area_name.contains("Hand") {
                if !self.ducked {
                    self.apply_knockback(area);
                    let this = self.to_gd();
                    let _guard = self.base_mut();
                    godot::task::spawn(Self::hitstun_routine(this));
                }
            } else {
                godot_print!("dodge");
            }
        } else {
            godot_print!("Touch!")
        }
    }
}

impl Player {
    fn movement(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut velocity = self.base().get_velocity();

        // Apply gravity
        velocity.y -= 20.0 * delta as f32;

        if !self.is_punching {
            velocity.z = 0.0;
        }
        if input.is_key_pressed(self.left_key) && !self.is_punching {
            velocity.z += self.speed;
            if self.facing_right {
                self.facing_right = false;
            }
        }
        if input.is_key_pressed(self.right_key) && !self.is_punching {
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
        if duck_pressed && on_floor {
            if !self.ducked {
                self.ducked = true;
            }
            velocity.z = 0.0;
        }
        if !duck_pressed && self.ducked {
            self.ducked = false;
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
            && !self.is_grab
        {
            scale.z *= -1.0;
            if let Some(ref mut collider) = self.body_collider {
                let mut collider_scale = collider.get_scale();
                collider_scale.z *= -1.0;
                collider.set_scale(collider_scale);
            }
            if let Some(ref mut label) = self.player_label {
                let mut label_scale = label.get_scale();
                label_scale.x *= -1.0;
                label.set_scale(label_scale);
            }
        }
        self.base_mut().set_scale(scale);
    }
    fn action_process(&mut self) {
        let input = Input::singleton();
        if input.is_key_pressed(self.punch_use_key) && !self.ducked {
            let this = self.to_gd();
            let _guard = self.base_mut();
            godot::task::spawn(Self::punch_routine(this));
        } else if input.is_key_pressed(self.throw_grab_ky) && !self.ducked {
            let this = self.to_gd();
            let _guard = self.base_mut();
            godot::task::spawn(Self::grab_routine(this));
        }
    }
    async fn punch_routine(mut this: Gd<Self>) {
        let original_speed;
        let mut speed_timer;
        {
            let mut bind = this.bind_mut();
            let mut velocity = bind.base().get_velocity();
            let direction = if bind.facing_right { -1.0 } else { 1.0 };
            velocity.z = direction * bind.punch_force;
            bind.base_mut().set_velocity(velocity);
            let mut t = Timer::new_alloc();
            t.set_wait_time(0.01);
            t.set_one_shot(true);
            speed_timer = t.clone();
            bind.base_mut().add_child(&t.upcast::<Node>());
        }
        let mut timer;
        {
            let mut bind = this.bind_mut();
            if bind.is_punching || bind.is_grab {
                return;
            }

            bind.is_punching = true;
            original_speed = bind.speed;
            bind.speed *= 0.0;
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
        speed_timer.start();
        timer.start();
        Signal::from_object_signal(&speed_timer, "timeout")
            .to_future::<()>()
            .await;
        {
            let mut bind = this.bind_mut();
            bind.speed = original_speed;
            let mut velocity = bind.base().get_velocity();
            velocity.z = 0.0;
            bind.base_mut().set_velocity(velocity);
        }
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;

        {
            let mut bind = this.bind_mut();
            bind.is_punching = false;
            if let Some(ref mut upper_anim) = bind.upper_anim_tree {
                upper_anim.set("parameters/conditions/l_punch", &false.to_variant());
                upper_anim.set("parameters/conditions/r_punch", &false.to_variant());
            }
        }
        speed_timer.upcast::<Node>().queue_free();
        timer.upcast::<Node>().queue_free();
    }
    async fn grab_routine(mut this: Gd<Self>) {
        let mut timer;
        {
            let mut bind = this.bind_mut();
            if bind.is_punching || bind.is_grab {
                return;
            }

            bind.is_grab = true;
            if let Some(ref mut upper_anim) = bind.upper_anim_tree {
                upper_anim.set("parameters/conditions/grab", &true.to_variant());
            }
            let mut t = Timer::new_alloc();
            t.set_wait_time(0.4);
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
            bind.is_grab = false;

            if let Some(ref mut upper_anim) = bind.upper_anim_tree {
                upper_anim.set("parameters/conditions/grab", &false.to_variant());
            }
        }
        timer.upcast::<Node>().queue_free();
    }
    async fn hitstun_routine(mut this: Gd<Self>) {
        let mut timer;
        {
            let mut bind = this.bind_mut();
            bind.hit_stun = true;
            if let Some(ref mut anim) = bind.lower_anim_tree {
                anim.set("parameters/conditions/hit", &true.to_variant());
            }
            if let Some(ref mut anim) = bind.upper_anim_tree {
                anim.set("parameters/conditions/hit", &true.to_variant());
            }
            let mut t = Timer::new_alloc();
            t.set_wait_time(0.5);
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
            bind.hit_stun = false;
            if let Some(ref mut anim) = bind.lower_anim_tree {
                anim.set("parameters/conditions/hit", &false.to_variant());
            }
            if let Some(ref mut anim) = bind.upper_anim_tree {
                anim.set("parameters/conditions/hit", &false.to_variant());
            }
            let mut velocity = bind.base().get_velocity();
            velocity.z = 0.0;
            bind.base_mut().set_velocity(velocity);
        }
        timer.upcast::<Node>().queue_free();
    }
    fn ready_body(&mut self) {
        self.body_collider = self
            .base()
            .find_child("PlayerCollider")
            .and_then(|node| node.try_cast::<CollisionShape3D>().ok());
        self.body_mesh = self
            .base()
            .find_child("PlayerHead")
            .and_then(|node| node.try_cast::<Node3D>().ok());
    }
    fn ready_animations(&mut self) {
        self.upper_anim_tree = self
            .base()
            .find_child("UpperAnim")
            .unwrap()
            .find_child("AnimationTree")
            .and_then(|node| node.try_cast::<AnimationTree>().ok());
        if let Some(ref mut anim_tree) = self.upper_anim_tree {
            anim_tree.set_active(true);
        }
        if let Some(ref mut anim_tree) = self.lower_anim_tree {
            anim_tree.set_active(true);
        }
        self.lower_anim_tree = self
            .base()
            .find_child("LowerAnim")
            .unwrap()
            .find_child("AnimationTree")
            .and_then(|node| node.try_cast::<AnimationTree>().ok());
    }
    fn ready_hitbox(&mut self) {
        self.hitbox = self
            .base()
            .find_child("PlayerHitBox")
            .and_then(|p| p.try_cast::<Area3D>().ok());
        let on_hit_callable = self.base().callable("on_player_hit");
        if let Some(ref mut hitbox) = self.hitbox {
            hitbox.connect("area_entered", &on_hit_callable);
        }
    }
    fn ready_label(&mut self) {
        self.player_label = self
            .base()
            .find_child(&format!("PlayerLabel"))
            .and_then(|f| f.try_cast::<Label3D>().ok());
        if let Some(ref mut label) = self.player_label {
            let label_string = format!("P{}", self.player_num.to_string());
            label.set_text(&label_string);
            label.set_modulate(utils::player_color_based_on_number(self.player_num));
        }
    }
    fn ready_groups(&mut self) {
        let left_hand_area = self.base().find_child("LeftHand").and_then(|f| {
            f.find_child("LeftHandArea")
                .and_then(|f| f.try_cast::<Area3D>().ok())
        });
        let grab_area = self.base().find_child("LeftHand").and_then(|f| {
            f.find_child("GrabArea")
                .and_then(|f| f.try_cast::<Area3D>().ok())
        });
        let right_hand_area = self.base().find_child("RightHand").and_then(|f| {
            f.find_child("RightHandArea")
                .and_then(|f| f.try_cast::<Area3D>().ok())
        });
        let _ = [left_hand_area, grab_area, right_hand_area].map(|mut f| {
            if let Some(ref mut u) = f {
                u.add_to_group(&format!("p{}", self.player_num));
            }
        });
    }
    fn apply_knockback(&mut self, area: Gd<Area3D>) {
        godot_print!("Hit!");
        let other_player_opt = area
            .get_parent()
            .and_then(|a| a.get_parent().and_then(|p| p.try_cast::<Player>().ok()));
        if let Some(other) = other_player_opt {
            let knockback_direction =
                self.base().get_global_position().z - other.get_global_position().z;
            let mut velocity = self.base().get_velocity();
            if knockback_direction > 0.0 {
                velocity.z = self.knock_back_force;
            } else {
                velocity.z = -1.0 * self.knock_back_force;
            }
            self.base_mut().set_velocity(velocity);
            self.base_mut().move_and_slide();
        }
    }
}
