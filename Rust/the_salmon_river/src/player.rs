use godot::classes::xr_positional_tracker::SignalsOfXrPositionalTracker;
use godot::classes::{
    AnimationTree, Area3D, CharacterBody3D, CollisionShape3D, GpuParticles3D, ICharacterBody3D,
    Input, Label3D, Sprite3D,
};
use godot::global::Key;
use godot::prelude::*;

use crate::throwable::Throwable;
use crate::throwables::throwability::Throwability;
use crate::utils::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    base: Base<CharacterBody3D>,
    speed: f32,
    jump_force: f32,
    punch_force: f32,
    jumped: bool,
    ducked: bool,
    hit_stun: bool,
    knock_back: bool,
    in_hand: bool,
    hit_stun_routine_entries: u8,
    hit_stun_hits: u8,
    hitstun_force: f32,
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
    duck_pafrticles: Option<Gd<GpuParticles3D>>,
    #[export]
    facing_right: bool,
    is_punching: bool,
    is_grab: bool,
    is_throwing: bool,
    right_punch: bool,
    #[export]
    #[var(pub)]
    player_num: u8,
    player_label: Option<Gd<Label3D>>,
    player_healthbar: Option<Gd<Sprite3D>>,
    pub health: u8,
    pub punch_damage: u8,
    initial_heealthbar_scale: f32,
    max_health: u8,
    throwable_in_hand: Option<Gd<Throwable>>,
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
            speed: 3.0,
            jump_force: 7.0,
            punch_force: 1.0,
            hitstun_force: 0.6,
            knock_back_force: 7.0,
            jumped: false,
            ducked: false,
            hit_stun: false,
            knock_back: false,
            in_hand: false,
            is_throwing: false,
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
            duck_pafrticles: None,
            hit_stun_routine_entries: 0,
            hit_stun_hits: 0,
            health: 9,
            max_health: 9,
            punch_damage: 1,
            player_healthbar: None,
            initial_heealthbar_scale: 0.0,
            throwable_in_hand: None,
        }
    }
    fn ready(&mut self) {
        self.ready_health_systems();
        self.ready_body();
        self.ready_animations();
        self.ready_hitbox();
        self.ready_label();
        self.ready_groups();
        self.ready_particle_system();
    }
    fn process(&mut self, delta: f64) {
        self.health_check();
        self.movement(delta);
        if !self.hit_stun && !self.knock_back {
            self.lower_animations();
            self.upper_animations();
            self.flip_based_on_facing_direction();
            self.action_process();
        }
    }
}
#[godot_api]
impl Player {
    #[signal]
    pub fn on_throwable_throw(dir: Direction);
    #[func]
    fn on_player_hit(&mut self, area: Gd<Area3D>) {
        let area_name = area.get_name();
        let area_groups = area.get_groups();
        if !area_groups.contains(&format!("p{}", self.player_num)) {
            godot_print!("{}", area_name);
            if area_name.contains("Grab") {
                godot_print!("Grab");
            } else if area_name.contains("Hand") {
                self.handle_getting_punched(area);
            } else if area_name.contains("ThrowableArea") {
                self.handle_throwable_hit(area);
            }
        } else {
            godot_print!("Touch!")
        }
    }
    #[func]
    pub fn search_for_throwable_in_hand(&mut self, player_num: u8) {
        if self.player_num == player_num {
            self.throwable_in_hand = self.base().find_child("RightHand").and_then(|rh| {
                rh.find_child("PickUpArea").and_then(|pua| {
                    pua.get_child(0)
                        .and_then(|th| th.try_cast::<Throwable>().ok())
                })
            });
        }
    }
}

impl Player {
    pub fn pick_up_throwable(&mut self) {
        self.in_hand = true;
        if let Some(ref mut upper_anim) = self.upper_anim_tree {
            upper_anim.set("parameters/conditions/in_hand", &true.to_variant());
        }
    }
    fn health_check(&mut self) {
        self.scale_healthbar_with_health();
        if self.health <= 0 {
            self.base_mut().queue_free();
        }
    }

    fn movement(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut velocity = self.base().get_velocity();
        // Apply gravity
        velocity.y -= 20.0 * delta as f32;
        if !self.is_punching && !self.hit_stun && !self.knock_back {
            velocity.z = 0.0;
        }
        if input.is_key_pressed(self.left_key) && !self.is_punching && !self.hit_stun {
            velocity.z += self.speed;
            if self.facing_right {
                self.facing_right = false;
            }
        }
        if input.is_key_pressed(self.right_key) && !self.is_punching && !self.hit_stun {
            velocity.z += -self.speed;
            if !self.facing_right {
                self.facing_right = true;
            }
        }
        if input.is_key_pressed(self.jump_key)
            && self.base().is_on_floor()
            && !self.jumped
            && !self.hit_stun
        {
            self.jumped = true;
            velocity.y = self.jump_force;
        }
        if !input.is_key_pressed(self.jump_key) {
            self.jumped = false;
        }
        let duck_pressed = input.is_key_pressed(self.duck_key);
        let on_floor = self.base().is_on_floor();
        if duck_pressed
            && on_floor
            && !self.hit_stun
            && !self.is_grab
            && !self.is_punching
            && !self.is_throwing
        {
            if !self.ducked {
                self.ducked = true;
                if let Some(ref mut particles) = self.duck_pafrticles {
                    particles.restart();
                }
            }
            velocity.z = 0.0;
        }
        if !duck_pressed && self.ducked {
            self.ducked = false;
            if let Some(ref mut particles) = self.duck_pafrticles {
                particles.restart();
            }
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
            if let Some(ref mut duck_particles) = self.duck_pafrticles {
                let mut particles_scale = duck_particles.get_scale();
                particles_scale.x *= -1.0;
                duck_particles.set_scale(particles_scale);
            }
        }
        self.base_mut().set_scale(scale);
    }
    fn action_process(&mut self) {
        let input = Input::singleton();
        if input.is_key_pressed(self.punch_use_key) && !self.ducked && !self.is_punching {
            if !self.in_hand {
                let this = self.to_gd();
                let _guard = self.base_mut();
                godot::task::spawn(Self::punch_routine(this));
            } else {
                if let Some(mut throwable) = self.throwable_in_hand.take() {
                    throwable.bind_mut().use_ability(self);
                    self.in_hand = false;
                    if let Some(ref mut upper_anim) = self.upper_anim_tree {
                        upper_anim.set("parameters/conditions/in_hand", &false.to_variant());
                    }
                }
            }
        } else if input.is_key_pressed(self.throw_grab_ky)
            && !self.ducked
            && !self.is_throwing
            && !self.is_grab
        {
            if !self.in_hand {
                let this = self.to_gd();
                let _guard = self.base_mut();
                godot::task::spawn(Self::grab_routine(this));
            } else {
                let this = self.to_gd();
                let _guard = self.base_mut();
                godot::task::spawn(Self::throw_routine(this));
            }
        }
    }
    async fn throw_routine(mut this: Gd<Self>) {
        let timer;
        {
            let mut bind = this.bind_mut();
            bind.is_throwing = true;
            if let Some(ref mut upper_anim) = bind.upper_anim_tree {
                upper_anim.set("parameters/conditions/in_hand", &false.to_variant());
                upper_anim.set("parameters/conditions/throw", &true.to_variant());
            }
            timer = bind.base().get_tree().create_timer(0.25);
        }
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;
        {
            let mut bind = this.bind_mut();
            let facing_right = bind.facing_right;
            if let Some(ref mut upper_anim) = bind.upper_anim_tree {
                upper_anim.set("parameters/conditions/in_hand", &false.to_variant());
                upper_anim.set("parameters/conditions/throw", &false.to_variant());
            }
            bind.signals().on_throwable_throw().emit({
                match facing_right {
                    true => Direction::Right,
                    false => Direction::Left,
                }
            });
            bind.in_hand = false;
            bind.throwable_in_hand = None;
            bind.is_throwing = false;
        }
    }
    async fn punch_routine(mut this: Gd<Self>) {
        let original_speed;
        let speed_timer;
        {
            let mut bind = this.bind_mut();
            let mut velocity = bind.base().get_velocity();
            let direction = if bind.facing_right { -1.0 } else { 1.0 };
            velocity.z = direction * bind.punch_force;
            bind.base_mut().set_velocity(velocity);
            speed_timer = bind.base().get_tree().create_timer(0.1);
        }
        let timer;
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
            timer = bind.base().get_tree().create_timer(0.25);
        }
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
    }
    async fn grab_routine(mut this: Gd<Self>) {
        let timer;
        {
            let mut bind = this.bind_mut();
            if bind.is_punching || bind.is_grab {
                return;
            }

            bind.is_grab = true;
            if let Some(ref mut upper_anim) = bind.upper_anim_tree {
                upper_anim.set("parameters/conditions/grab", &true.to_variant());
            }
            timer = bind.base().get_tree().create_timer(0.4);
        }

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
    }
    async fn hitstun_routine(mut this: Gd<Self>, knock_dir: Direction) {
        let should_knockback;
        {
            let mut bind = this.bind_mut();
            if bind.hit_stun {
                bind.hit_stun_routine_entries += 1;
                bind.hit_stun_hits += 1;
            }
            should_knockback = bind.hit_stun_hits > 1;
            bind.hit_stun = true;
        }
        if should_knockback {
            godot::task::spawn(Self::knockback_routine(this, knock_dir));
            return;
        }
        let timer;
        {
            let mut bind = this.bind_mut();
            if let Some(ref mut anim) = bind.lower_anim_tree {
                anim.set("parameters/conditions/hit", &true.to_variant());
            }
            if let Some(ref mut anim) = bind.upper_anim_tree {
                anim.set("parameters/conditions/hit", &true.to_variant());
            }
            timer = bind.base().get_tree().create_timer(0.5);
        }
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;
        {
            let mut bind = this.bind_mut();
            if bind.hit_stun_routine_entries > 0 {
                bind.hit_stun_routine_entries -= 1;
            } else {
                bind.hit_stun = false;
                bind.hit_stun_hits = 0;
            }
            if let Some(ref mut anim) = bind.lower_anim_tree {
                anim.set("parameters/conditions/hit", &false.to_variant());
            }
            if let Some(ref mut anim) = bind.upper_anim_tree {
                anim.set("parameters/conditions/hit", &false.to_variant());
            }
            let mut velocity = bind.base().get_velocity();
            if !bind.knock_back {
                velocity.z = 0.0;
                bind.base_mut().set_velocity(velocity);
            }
        }
    }
    async fn knockback_routine(mut this: Gd<Self>, knock_dir: Direction) {
        let timer;
        {
            let mut bind = this.bind_mut();
            bind.knock_back = true;
            let mut velocity = bind.base().get_velocity();
            velocity.y += bind.knock_back_force;
            match knock_dir {
                Direction::Left => velocity.z -= bind.knock_back_force * 0.4,
                Direction::Right => velocity.z += bind.knock_back_force * 0.4,
            }
            bind.base_mut().set_velocity(velocity);
            if let Some(ref mut low_anim) = bind.lower_anim_tree {
                low_anim.set("parameters/conditions/jump", &false.to_variant());
                low_anim.set("parameters/conditions/idle", &false.to_variant());
                low_anim.set("parameters/conditions/run", &false.to_variant());
                low_anim.set("parameters/conditions/duck", &false.to_variant());
                low_anim.set("parameters/conditions/hit", &false.to_variant());
                low_anim.set("parameters/conditions/knock", &true.to_variant());
            }
            if let Some(ref mut upp_anim) = bind.upper_anim_tree {
                upp_anim.set("parameters/conditions/jump", &false.to_variant());
                upp_anim.set("parameters/conditions/idle", &false.to_variant());
                upp_anim.set("parameters/conditions/run", &false.to_variant());
                upp_anim.set("parameters/conditions/hit", &false.to_variant());
                upp_anim.set("parameters/conditions/knock", &true.to_variant());
            }
            timer = bind.base().get_tree().create_timer(1.0);
        }
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;
        {
            let mut bind = this.bind_mut();
            let mut velocity = bind.base().get_velocity();
            velocity.z = 0.0;
            bind.base_mut().set_velocity(velocity);
            bind.knock_back = false;
            bind.hit_stun_hits = 0;
            bind.hit_stun = false;
            if let Some(ref mut low_anim) = bind.lower_anim_tree {
                low_anim.set("parameters/conditions/hit", &false.to_variant());
                low_anim.set("parameters/conditions/knock", &false.to_variant());
            }
            if let Some(ref mut upp_anim) = bind.upper_anim_tree {
                upp_anim.set("parameters/conditions/knock", &false.to_variant());
            }
        }
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
            .and_then(|node| node.find_child("AnimationTree"))
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
            .and_then(|node| node.find_child("AnimationTree"))
            .and_then(|node| node.try_cast::<AnimationTree>().ok());
    }
    fn ready_hitbox(&mut self) {
        self.hitbox = self
            .base()
            .find_child("PlayerHitBox")
            .and_then(|p| p.try_cast::<Area3D>().ok());
        let this = self.to_gd();
        if let Some(ref mut hitbox) = self.hitbox {
            hitbox
                .signals()
                .area_entered()
                .connect_other(&this, Self::on_player_hit);
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
            label.set_modulate(player_color_based_on_number(self.player_num));
        }
    }
    fn ready_groups(&mut self) {
        let left_hand_area = self.base().find_child("LeftHand").and_then(|f| {
            f.find_child("LeftHandArea")
                .and_then(|f| f.try_cast::<Area3D>().ok())
        });
        let grab_area = self.base().find_child("LeftHand").and_then(|f| {
            f.find_child("ThrowableArea")
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
    fn ready_particle_system(&mut self) {
        self.duck_pafrticles = self
            .base()
            .find_child("DuckPoofParticles")
            .and_then(|p| p.try_cast::<GpuParticles3D>().ok());
    }
    fn ready_health_systems(&mut self) {
        self.health = self.max_health;
        self.player_healthbar = self
            .base()
            .find_child("PlayerLabel")
            .and_then(|f| f.find_child("HealthBar"))
            .and_then(|hb| hb.try_cast::<Sprite3D>().ok());
        if let Some(ref mut healthbar) = self.player_healthbar {
            self.initial_heealthbar_scale = healthbar.get_scale().x;
            healthbar.set_modulate(crate::utils::player_color_based_on_number(self.player_num));
        }
    }
    fn scale_healthbar_with_health(&mut self) {
        if let Some(ref mut healthbar) = self.player_healthbar {
            let health_ratio = self.health as f32 / self.max_health as f32;
            let mut hb_scale = healthbar.get_scale();
            hb_scale.x = self.initial_heealthbar_scale * health_ratio;
            healthbar.set_scale(hb_scale);
        }
    }
    fn apply_hitstun_force(&mut self, area: Gd<Area3D>) -> Option<Direction> {
        godot_print!("Hit!");
        let knockback_direction = {
            let other_player_opt = area
                .get_parent()
                .and_then(|a| a.get_parent().and_then(|p| p.try_cast::<Player>().ok()));
            if let Some(other) = other_player_opt {
                self.base().get_global_position().z - other.get_global_position().z
            } else {
                self.base().get_global_position().z - area.get_global_position().z
            }
        };
        let mut velocity = self.base().get_velocity();
        let mut to_ret = None;
        if knockback_direction > 0.0 {
            velocity.z = self.hitstun_force;
            to_ret = Some(Direction::Right);
        } else {
            velocity.z = -1.0 * self.hitstun_force;
            to_ret = Some(Direction::Left);
        }
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
        return to_ret;
    }
    fn handle_getting_punched(&mut self, area: Gd<Area3D>) {
        if !self.ducked {
            let player_opt = area
                .get_parent()
                .and_then(|hand| hand.get_parent())
                .and_then(|player| player.try_cast::<Player>().ok());
            let knock_dir_opt = self.apply_hitstun_force(area);
            if let Some(player) = player_opt {
                self.health -= player.bind().punch_damage;
            }
            if let Some(knock_dir) = knock_dir_opt {
                let this = self.to_gd();
                let _guard = self.base_mut();
                godot::task::spawn(Self::hitstun_routine(this, knock_dir));
            }
        }
    }
    fn handle_throwable_hit(&mut self, area: Gd<Area3D>) {
        let throwable_opt = area
            .get_parent()
            .and_then(|th| th.try_cast::<Throwable>().ok());

        if !self.ducked
            && let Some(mut throwable) = throwable_opt
            && throwable.bind().does_player_hitstun(self.player_num)
        {
            if let Some(ref mut inner) = throwable.bind_mut().throwable_inner {
                self.health -= inner.deal_dmg();
            }
            let knock_dir_opt = self.apply_hitstun_force(area);
            if let Some(knock_dir) = knock_dir_opt {
                let this = self.to_gd();
                let _guard = self.base_mut();
                godot::task::spawn(Self::hitstun_routine(this, knock_dir));
            }
        }
    }
}
