use godot::classes::{
    AnimationTree, Area3D, CharacterBody3D, CollisionShape3D, Engine, GpuParticles3D,
    ICharacterBody3D, Input, Label3D, Sprite3D, Time, VisualShaderNodeGroupBase,
};
use godot::global::Key;
use godot::prelude::*;
use godot::signal::ConnectHandle;

use crate::duck_meter_manager::DuckMeterManager;
use crate::throwable::Throwable;
use crate::throwables::throwability::Throwability;
use crate::utils::Direction::Left;
use crate::utils::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    base: Base<CharacterBody3D>,
    speed: f32,
    jump_force: f32,
    punch_force: f32,
    ice_chunk_dash_force: f32,
    jumped: bool,
    duck_jumped: bool,
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
    is_using_throwable_ability: bool,
    is_dashing: bool,
    right_punch: bool,
    #[export]
    #[var(pub)]
    player_num: u8,
    player_label: Option<Gd<Label3D>>,
    player_healthbar: Option<Gd<Sprite3D>>,
    player_duck_bar: Option<Gd<Sprite3D>>,
    pub health: u8,
    punch_damage: u8,
    additional_next_punch_damage: u8,
    initial_heealthbar_scale: f32,
    intitial_duck_bar_scale: f32,
    max_health: u8,
    throwable_in_hand: Option<Gd<Throwable>>,
    player_throwable_in_hand: Option<Gd<Player>>,
    duck_meter_manager: DuckMeterManager,
    duck_jumping: bool,
    dead: bool,
    ice_chunk_dash_particles: Option<Gd<GpuParticles3D>>,
    left_hand_fire_particles: Option<Gd<GpuParticles3D>>,
    right_hand_fire_particles: Option<Gd<GpuParticles3D>>,
    grabbed_by_another_player: bool,
    thrown_by_another_player: bool,
    on_thrown_by_another_conn: Option<ConnectHandle>,
    force_on_thrown: f32,
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
            ice_chunk_dash_force: 10.0,
            hitstun_force: 0.6,
            knock_back_force: 7.0,
            jumped: false,
            duck_jumped: false,
            ducked: false,
            hit_stun: false,
            knock_back: false,
            in_hand: false,
            is_throwing: false,
            is_dashing: false,
            is_using_throwable_ability: false,
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
            additional_next_punch_damage: 0,
            player_healthbar: None,
            initial_heealthbar_scale: 0.0,
            intitial_duck_bar_scale: 0.0,
            throwable_in_hand: None,
            player_duck_bar: None,
            duck_meter_manager: DuckMeterManager::new(100, 2, 5, 50),
            duck_jumping: false,
            dead: false,
            ice_chunk_dash_particles: None,
            left_hand_fire_particles: None,
            right_hand_fire_particles: None,
            grabbed_by_another_player: false,
            thrown_by_another_player: false,
            on_thrown_by_another_conn: None,
            force_on_thrown: 8.0,
            player_throwable_in_hand: None,
        }
    }
    fn ready(&mut self) {
        self.ready_throwable_effect_particles();
        self.ready_health_systems();
        self.ready_duck_meter_systems();
        self.ready_body();
        self.ready_animations();
        self.ready_hitbox();
        self.ready_label();
        self.ready_groups();
        self.ready_particle_system();
    }
    fn process(&mut self, delta: f64) {
        self.health_check();
        self.negate_scale_changes_on_grab();
        if !self.grabbed_by_another_player {
            self.movement(delta);
        }
        if !self.hit_stun && !self.knock_back && !self.dead && !self.grabbed_by_another_player {
            self.lower_animations();
            self.upper_animations();
            self.flip_based_on_facing_direction();
            self.action_process();
        }
        self.duck_bar_systems();
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
                self.handle_grab(area);
            } else if area_name.contains("Hand") {
                self.handle_punch(area);
            } else if area_name.contains("ThrowableArea") {
                self.handle_throwable_hit(area);
            }
        } else {
            godot_print!("Touch!")
        }
    }
}

impl Player {
    pub fn is_dead(&self) -> bool {
        self.dead
    }
    pub fn duck_bar_systems(&mut self) {
        self.duck_meter_manager.duck_meter_update(
            self.duck_jumped,
            self.ducked,
            Time::singleton().get_ticks_msec(),
        );
        self.scale_duckbar();
    }
    pub fn pick_up_throwable(&mut self) {
        self.in_hand = true;
        if let Some(ref mut upper_anim) = self.upper_anim_tree {
            upper_anim.set("parameters/conditions/in_hand", &true.to_variant());
            upper_anim.set("parameters/conditions/drop", &false.to_variant());
        }
    }
    pub fn hold_throwable(&mut self, throwable: Gd<Throwable>) {
        self.throwable_in_hand = Some(throwable);
    }
    pub fn drop_throwable(&mut self) {
        self.in_hand = false;
        if let Some(mut throwable) = self.throwable_in_hand.take() {
            if throwable.is_instance_valid() {
                throwable.bind_mut().drop_itself();
            }
        }
        if let Some(mut player_throwable) = self.player_throwable_in_hand.take() {
            if player_throwable.is_instance_valid() {
                player_throwable.bind_mut().on_dropped_by_another_player();
            }
        }
        if let Some(ref mut upper_anim) = self.upper_anim_tree {
            upper_anim.set("parameters/conditions/in_hand", &false.to_variant());
            upper_anim.set("parameters/conditions/drop", &true.to_variant());
        }
    }
    pub fn can_pick_up_throwable(&self) -> bool {
        !self.in_hand
    }
    pub fn heal(&mut self, amount: u8) {
        self.health = (self.health + amount).min(self.max_health);
    }
    pub fn damage(&mut self, amount: u8) {
        self.health = self.health.saturating_sub(amount);
    }
    fn negate_scale_changes_on_grab(&mut self) {
        if self.grabbed_by_another_player {
            self.base_mut().set_scale(Vector3::ONE);
            self.base_mut().set_rotation(Vector3::ZERO);
        }
    }

    fn health_check(&mut self) {
        self.scale_healthbar_with_health();
        // if self.health <= 0 {
        //     self.base_mut().queue_free();
        // }
    }
    fn movement(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut velocity = self.base().get_velocity();
        // Apply gravity
        velocity.y -= 20.0 * delta as f32;
        let mut speed_to_use = self.speed;
        if self.player_throwable_in_hand.is_some() {
            speed_to_use *= 0.5;
        }
        if !self.is_punching && !self.hit_stun && !self.knock_back && !self.is_dashing && !self.dead
        {
            velocity.z = 0.0;
        }
        if input.is_key_pressed(self.left_key)
            && !self.is_punching
            && !self.hit_stun
            && !self.is_dashing
            && !self.dead
            && !self.knock_back
        {
            velocity.z += speed_to_use;
            if self.facing_right {
                self.facing_right = false;
            }
        }
        if input.is_key_pressed(self.right_key)
            && !self.is_punching
            && !self.hit_stun
            && !self.is_dashing
            && !self.dead
            && !self.knock_back
        {
            velocity.z += -speed_to_use;
            if !self.facing_right {
                self.facing_right = true;
            }
        }
        if input.is_key_pressed(self.jump_key)
            && self.base().is_on_floor()
            && !self.jumped
            && !self.hit_stun
            && !self.dead
            && !self.knock_back
        {
            self.jumped = true;
            velocity.y = self.jump_force;
        }
        if !input.is_key_pressed(self.jump_key) {
            self.jumped = false;
        }
        if input.is_key_pressed(self.duck_key)
            && !self.base().is_on_floor()
            && !self.duck_jumped
            && !self.dead
            && !self.knock_back
            && self.duck_meter_manager.can_duck_jump()
        {
            velocity.y = self.jump_force;
            self.duck_jumped = true;
            let this = self.to_gd();
            let _guard = self.base_mut();
            godot::task::spawn(Self::duck_jump_routine(this));
        }
        if self.base().is_on_floor() {
            self.duck_jumped = false;
        }
        if input.is_key_pressed(self.duck_key)
            && self.base().is_on_floor()
            && !self.hit_stun
            && !self.is_grab
            && !self.is_punching
            && !self.is_throwing
            && !self.dead
            && !self.knock_back
            && self.duck_meter_manager.can_duck()
        {
            if !self.ducked {
                self.ducked = true;
                if let Some(ref mut particles) = self.duck_pafrticles {
                    particles.restart();
                }
            }
            velocity.z = 0.0;
        }
        if !input.is_key_pressed(self.duck_key) && self.ducked
            || !self.duck_meter_manager.can_duck() && self.ducked
        {
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
        let scale = self.base().get_scale();
        if (self.facing_right && scale.z < 0.0 || !self.facing_right && scale.z > 0.0)
            && !self.is_punching
            && !self.is_grab
        {
            self.flip();
        }
    }
    fn flip(&mut self) {
        let mut scale = self.base().get_scale();
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
        self.base_mut().set_scale(scale);
    }
    fn action_process(&mut self) {
        let input = Input::singleton();
        if input.is_key_pressed(self.punch_use_key)
            && !self.ducked
            && !self.is_punching
            && !self.is_using_throwable_ability
        {
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
                        upper_anim.set("parameters/conditions/drop", &true.to_variant());
                    }
                }
                let this = self.to_gd();
                let _guard = self.base_mut();
                godot::task::spawn(Self::use_routine(this));
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
    async fn duck_jump_routine(mut this: Gd<Self>) {
        let timer;
        {
            let mut bind = this.bind_mut();
            bind.duck_jumping = true;
            if let Some(ref mut duck_particles) = bind.duck_pafrticles {
                duck_particles.restart();
            }
            if let Some(ref mut lower_anim) = bind.lower_anim_tree {
                lower_anim.set("parameters/conditions/duck_jump", &true.to_variant());
            }
            timer = bind.base().get_tree().create_timer(0.5);
        }
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;
        {
            let mut bind = this.bind_mut();
            bind.duck_jumping = false;
            if let Some(ref mut duck_particles) = bind.duck_pafrticles {
                duck_particles.restart();
            }
            if let Some(ref mut lower_anim) = bind.lower_anim_tree {
                lower_anim.set("parameters/conditions/duck_jump", &false.to_variant());
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
            bind.drop_throwable();
            bind.in_hand = false;
            bind.throwable_in_hand = None;
            bind.is_throwing = false;
        }
    }
    async fn use_routine(mut this: Gd<Self>) {
        let timer;
        {
            let mut bind = this.bind_mut();
            bind.is_using_throwable_ability = true;
            timer = bind.base().get_tree().create_timer(0.1);
        }
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;
        {
            let mut bind = this.bind_mut();
            bind.is_using_throwable_ability = false;
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
    pub fn spawn_log_obsticle(&self) {
        let scene = load::<PackedScene>("res://Prefabs/log_obsticle.tscn");
        if let Some(mut log_obsticle) = scene
            .instantiate()
            .and_then(|log| log.try_cast::<Node3D>().ok())
        {
            let mut log_pos = self.base().get_position();
            match self.facing_right {
                true => log_pos.z -= 1.0,
                _ => log_pos.z += 1.0,
            }
            log_pos.y += 2.0;
            log_obsticle.set_position(log_pos);
            if let Some(mut root) = self.base().get_tree().get_current_scene() {
                root.add_child(&log_obsticle);
            }
            let timer = self.base().get_tree().create_timer(4.0);
            godot::task::spawn(async move {
                Signal::from_object_signal(&timer, "timeout")
                    .to_future::<()>()
                    .await;
                log_obsticle.call_deferred("queue_free", &[]);
            });
        }
    }
    pub async fn ice_chunk_dash_routine(mut this: Gd<Self>) {
        let dash_timer;
        {
            let mut bind = this.bind_mut();
            if let Some(ref mut particles) = bind.ice_chunk_dash_particles {
                particles.set_emitting(true);
            }
            bind.is_dashing = true;
            let mut velocity = bind.base().get_velocity();
            let direction = if bind.facing_right { -1.0 } else { 1.0 };
            velocity.z = direction * bind.ice_chunk_dash_force;
            bind.base_mut().set_velocity(velocity);
            dash_timer = bind.base().get_tree().create_timer(0.2);
        }
        Signal::from_object_signal(&dash_timer, "timeout")
            .to_future::<()>()
            .await;
        {
            let mut bind = this.bind_mut();
            if let Some(ref mut particles) = bind.ice_chunk_dash_particles {
                particles.set_emitting(false);
            }
            bind.is_dashing = false;
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
            if bind.knock_back {
                return;
            }
            if bind.hit_stun {
                bind.hit_stun_routine_entries += 1;
                bind.hit_stun_hits += 1;
            }
            should_knockback = bind.hit_stun_hits > 1;
            bind.hit_stun = true;
        }
        if should_knockback {
            godot::task::spawn(Self::knockback_routine(this, knock_dir, false));
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
    async fn death_routine(mut this: Gd<Self>, knock_dir: Direction) {
        let timer;
        {
            let mut bind = this.bind_mut();
            bind.dead = true;
            let mut velocity = bind.base().get_velocity();
            bind.drop_throwable();
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
                low_anim.set("parameters/conditions/knock", &false.to_variant());
                low_anim.set("parameters/conditions/death", &true.to_variant());
            }
            if let Some(ref mut upp_anim) = bind.upper_anim_tree {
                upp_anim.set("parameters/conditions/jump", &false.to_variant());
                upp_anim.set("parameters/conditions/idle", &false.to_variant());
                upp_anim.set("parameters/conditions/run", &false.to_variant());
                upp_anim.set("parameters/conditions/hit", &false.to_variant());
                upp_anim.set("parameters/conditions/knock", &false.to_variant());
                upp_anim.set("parameters/conditions/death", &true.to_variant());
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
            if let Some(ref mut low_anim) = bind.lower_anim_tree {
                low_anim.set("parameters/conditions/jump", &false.to_variant());
                low_anim.set("parameters/conditions/idle", &false.to_variant());
                low_anim.set("parameters/conditions/run", &false.to_variant());
                low_anim.set("parameters/conditions/duck", &false.to_variant());
                low_anim.set("parameters/conditions/hit", &false.to_variant());
                low_anim.set("parameters/conditions/knock", &false.to_variant());
                low_anim.set("parameters/conditions/death", &false.to_variant());
            }
            if let Some(ref mut upp_anim) = bind.upper_anim_tree {
                upp_anim.set("parameters/conditions/jump", &false.to_variant());
                upp_anim.set("parameters/conditions/idle", &false.to_variant());
                upp_anim.set("parameters/conditions/run", &false.to_variant());
                upp_anim.set("parameters/conditions/hit", &false.to_variant());
                upp_anim.set("parameters/conditions/knock", &false.to_variant());
                upp_anim.set("parameters/conditions/death", &false.to_variant());
            }
        }
    }
    async fn throw_landing_damage_routine(mut this: Gd<Self>) {
        while !this.bind().base().is_on_floor() {
            let timer = this.bind().base().get_tree().create_timer(0.02);
            Signal::from_object_signal(&timer, "timeout")
                .to_future::<()>()
                .await;
        }
        this.bind_mut().damage(2);
    }
    async fn knockback_routine(mut this: Gd<Self>, knock_dir: Direction, from_throw: bool) {
        let timer;
        {
            let mut bind = this.bind_mut();
            bind.knock_back = true;
            let mut velocity = bind.base().get_velocity();
            bind.drop_throwable();
            let mut knock_back_force_used = bind.knock_back_force;
            if from_throw {
                knock_back_force_used *= 1.5;
            }
            velocity.y += knock_back_force_used;
            match knock_dir {
                Direction::Left => velocity.z -= knock_back_force_used * 0.4,
                Direction::Right => velocity.z += knock_back_force_used * 0.4,
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
            timer = bind.base().get_tree().create_timer(1.3);
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
    fn ready_particle_system(&mut self) {
        self.duck_pafrticles = self
            .base()
            .find_child("DuckPoofParticles")
            .and_then(|p| p.try_cast::<GpuParticles3D>().ok());
    }
    fn ready_health_systems(&mut self) {
        self.heal(self.max_health);
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
    fn ready_duck_meter_systems(&mut self) {
        self.player_duck_bar = self
            .base()
            .find_child("PlayerLabel")
            .and_then(|f| f.find_child("DuckBar"))
            .and_then(|db| db.try_cast::<Sprite3D>().ok());
        if let Some(ref mut duckbar) = self.player_duck_bar {
            self.intitial_duck_bar_scale = duckbar.get_scale().x;
            duckbar.set_modulate(crate::utils::complementary_color(
                crate::utils::player_color_based_on_number(self.player_num),
            ));
        }
    }
    fn ready_throwable_effect_particles(&mut self) {
        self.ice_chunk_dash_particles = self.base().find_child("PlayerHead").and_then(|head| {
            head.find_child("IceChunkDashParticles")
                .and_then(|p| p.try_cast::<GpuParticles3D>().ok())
        });
        self.left_hand_fire_particles = self.base().find_child("LeftHand").and_then(|hand| {
            hand.find_child("LeftHandMesh").and_then(|mesh| {
                mesh.find_child("BuffParticlesHand")
                    .and_then(|bph| bph.try_cast::<GpuParticles3D>().ok())
            })
        });
        self.right_hand_fire_particles = self.base().find_child("RightHand").and_then(|hand| {
            hand.find_child("RightHandMesh").and_then(|mesh| {
                mesh.find_child("BuffParticlesHand")
                    .and_then(|bph| bph.try_cast::<GpuParticles3D>().ok())
            })
        });
    }
    fn scale_healthbar_with_health(&mut self) {
        if let Some(ref mut healthbar) = self.player_healthbar {
            let health_ratio = self.health as f32 / self.max_health as f32;
            let mut hb_scale = healthbar.get_scale();
            hb_scale.x = self.initial_heealthbar_scale * health_ratio;
            healthbar.set_scale(hb_scale);
        }
    }
    fn scale_duckbar(&mut self) {
        if let Some(ref mut duckbar) = self.player_duck_bar {
            let duck_bar_ratio = self.duck_meter_manager.get_duck_meter() as f32
                / self.duck_meter_manager.get_max_duck_meter() as f32;
            let mut hb_scale = duckbar.get_scale();
            hb_scale.x = self.initial_heealthbar_scale * duck_bar_ratio;
            duckbar.set_scale(hb_scale);
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
            if !self.facing_right {
                self.flip();
                self.facing_right = true;
            }
        } else {
            velocity.z = -1.0 * self.hitstun_force;
            to_ret = Some(Direction::Left);
            if self.facing_right {
                self.flip();
                self.facing_right = false;
            }
        }
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
        return to_ret;
    }
    fn calculate_punch_damage(&mut self) -> u8 {
        let to_ret = self.punch_damage + self.additional_next_punch_damage;
        self.additional_next_punch_damage = 0;
        if let Some(ref mut right_particles) = self.right_hand_fire_particles {
            right_particles.set_emitting(false);
        }
        if let Some(ref mut left_particles) = self.left_hand_fire_particles {
            left_particles.set_emitting(false);
        }
        to_ret
    }
    pub fn add_additional_punch_damage(&mut self, amount: u8) {
        self.additional_next_punch_damage += amount;
        if let Some(ref mut right_particles) = self.right_hand_fire_particles {
            right_particles.set_emitting(true);
        }
        if let Some(ref mut left_particles) = self.left_hand_fire_particles {
            left_particles.set_emitting(true);
        }
    }
    fn disconnect_grabbed_connections(&mut self) {
        if let Some(handle) = self.on_thrown_by_another_conn.take() {
            if handle.is_connected() {
                handle.disconnect();
            }
        }
    }
    fn on_dropped_by_another_player(&mut self) {
        self.disconnect_grabbed_connections();
        let scene_root_opt = self.base().get_tree().get_current_scene();
        if let Some(scene_root) = scene_root_opt {
            self.base_mut()
                .call_deferred("reparent", &[scene_root.to_variant()]);
        }
        if let Some(ref mut low_anim) = self.lower_anim_tree {
            low_anim.set("parameters/conditions/jump", &false.to_variant());
            low_anim.set("parameters/conditions/idle", &false.to_variant());
            low_anim.set("parameters/conditions/run", &false.to_variant());
            low_anim.set("parameters/conditions/duck", &false.to_variant());
            low_anim.set("parameters/conditions/hit", &false.to_variant());
            low_anim.set("parameters/conditions/knock", &false.to_variant());
        }
        if let Some(ref mut upp_anim) = self.upper_anim_tree {
            upp_anim.set("parameters/conditions/jump", &false.to_variant());
            upp_anim.set("parameters/conditions/idle", &false.to_variant());
            upp_anim.set("parameters/conditions/run", &false.to_variant());
            upp_anim.set("parameters/conditions/hit", &false.to_variant());
            upp_anim.set("parameters/conditions/knock", &false.to_variant());
        }
        self.grabbed_by_another_player = false;
    }
    fn on_thrown_by_another_player(&mut self, dir: Direction) {
        self.disconnect_grabbed_connections();
        if let Some(ref mut low_anim) = self.lower_anim_tree {
            low_anim.set("parameters/conditions/jump", &false.to_variant());
            low_anim.set("parameters/conditions/idle", &false.to_variant());
            low_anim.set("parameters/conditions/run", &false.to_variant());
            low_anim.set("parameters/conditions/duck", &false.to_variant());
            low_anim.set("parameters/conditions/hit", &false.to_variant());
            low_anim.set("parameters/conditions/knock", &false.to_variant());
        }
        if let Some(ref mut upp_anim) = self.upper_anim_tree {
            upp_anim.set("parameters/conditions/jump", &false.to_variant());
            upp_anim.set("parameters/conditions/idle", &false.to_variant());
            upp_anim.set("parameters/conditions/run", &false.to_variant());
            upp_anim.set("parameters/conditions/hit", &false.to_variant());
            upp_anim.set("parameters/conditions/knock", &false.to_variant());
        }
        self.thrown_by_another_player = true;
        let scene_root_opt = self.base().get_tree().get_current_scene();
        if let Some(scene_root) = scene_root_opt {
            self.base_mut()
                .call_deferred("reparent", &[scene_root.to_variant()]);
        }
        self.grabbed_by_another_player = false;
        self.base_mut().set_velocity(Vector3::ZERO);
        self.base_mut().set_rotation(Vector3::ZERO);
        self.base_mut().set_scale(Vector3::ONE);
        let this = self.to_gd();
        let _guard = self.base_mut();
        godot::task::spawn(Self::knockback_routine(
            this.clone(),
            {
                match dir {
                    Direction::Left => Direction::Right,
                    _ => Direction::Left,
                }
            },
            true,
        ));
        godot::task::spawn(Self::throw_landing_damage_routine(this));
    }
    fn handle_grab(&mut self, area: Gd<Area3D>) {
        if !self.knock_back {
            let other_player_opt = area.get_parent().and_then(|left_hand| {
                left_hand
                    .get_parent()
                    .and_then(|player| player.try_cast::<Player>().ok())
            });
            if let Some(mut grabber) = other_player_opt {
                if grabber.bind().can_pick_up_throwable() {
                    grabber.bind_mut().pick_up_throwable();
                    self.disconnect_grabbed_connections();
                    let this = self.to_gd();
                    self.on_thrown_by_another_conn = Some(
                        grabber
                            .signals()
                            .on_throwable_throw()
                            .connect_other(&this, Self::on_thrown_by_another_player),
                    );
                    grabber.bind_mut().player_throwable_in_hand = Some(this);
                    let pickup_area_opt = grabber.find_child("RightHand").and_then(|rh| {
                        rh.find_child("PickUpArea")
                            .and_then(|pua| pua.try_cast::<Node3D>().ok())
                    });
                    if let Some(pickup_area) = pickup_area_opt {
                        self.base_mut()
                            .call_deferred("reparent", &[pickup_area.to_variant()]);
                        let mut local_pos = Vector3::ZERO;
                        local_pos.y -= 1.5;
                        self.base_mut()
                            .call_deferred("set_position", &[local_pos.to_variant()]);
                        self.grabbed_by_another_player = true;
                    }
                    if let Some(ref mut low_anim) = self.lower_anim_tree {
                        low_anim.set("parameters/conditions/jump", &false.to_variant());
                        low_anim.set("parameters/conditions/idle", &false.to_variant());
                        low_anim.set("parameters/conditions/run", &false.to_variant());
                        low_anim.set("parameters/conditions/duck", &false.to_variant());
                        low_anim.set("parameters/conditions/hit", &false.to_variant());
                        low_anim.set("parameters/conditions/knock", &true.to_variant());
                    }
                    if let Some(ref mut upp_anim) = self.upper_anim_tree {
                        upp_anim.set("parameters/conditions/jump", &false.to_variant());
                        upp_anim.set("parameters/conditions/idle", &false.to_variant());
                        upp_anim.set("parameters/conditions/run", &false.to_variant());
                        upp_anim.set("parameters/conditions/hit", &false.to_variant());
                        upp_anim.set("parameters/conditions/knock", &true.to_variant());
                    }
                    let drop_timer = self.base().get_tree().create_timer(2.0);
                    godot::task::spawn(async move {
                        Signal::from_object_signal(&drop_timer, "timeout")
                            .to_future::<()>()
                            .await;
                        grabber.bind_mut().drop_throwable();
                    });
                }
            }
        }
    }
    fn handle_punch(&mut self, area: Gd<Area3D>) {
        if !self.ducked && !self.duck_jumping {
            let player_opt = area
                .get_parent()
                .and_then(|hand| hand.get_parent())
                .and_then(|player| player.try_cast::<Player>().ok());
            let knock_dir_opt = self.apply_hitstun_force(area);
            if let Some(mut player) = player_opt {
                self.damage(player.bind_mut().calculate_punch_damage());
            }
            if let Some(knock_dir) = knock_dir_opt {
                let health = self.health;
                let this = self.to_gd();
                let _guard = self.base_mut();
                if health <= 0 {
                    godot::task::spawn(Self::death_routine(this, knock_dir));
                } else {
                    godot::task::spawn(Self::hitstun_routine(this, knock_dir));
                }
            }
        }
    }
    fn handle_throwable_hit(&mut self, area: Gd<Area3D>) {
        let throwable_opt = area
            .get_parent()
            .and_then(|th| th.try_cast::<Throwable>().ok());

        if !self.ducked
            && !self.duck_jumping
            && let Some(mut throwable) = throwable_opt
            && throwable.bind().does_player_hitstun(self.player_num)
        {
            if let Some(ref mut inner) = throwable.bind_mut().throwable_inner {
                self.damage(inner.deal_dmg());
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
