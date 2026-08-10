use crate::{
    game_scripts::{game_utils::ThrowableSpawnDirection, throwable::Throwable},
    throwables::{ice_chunk::IceChunk, log::Log, salmon::Salmon},
};
use godot::{
    classes::{AnimationTree, RandomNumberGenerator, Time},
    prelude::*,
};
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ThrowableSpawner {
    base: Base<Node3D>,
    throw_force: f32,
    next_spawn_time_ms: u64,
    spawn_rate_ms: u64,
    arrow_anim: Option<Gd<AnimationTree>>,
    next_arrow_position: ThrowableSpawnDirection,
    ms_before_arrow_direction_show: u64,
}
#[godot_api]
impl INode3D for ThrowableSpawner {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base: base,
            throw_force: 5.0,
            next_spawn_time_ms: 0,
            spawn_rate_ms: 8000,
            arrow_anim: None,
            next_arrow_position: ThrowableSpawnDirection::Up,
            ms_before_arrow_direction_show: 4000,
        }
    }
    fn ready(&mut self) {
        self.next_spawn_time_ms = self.spawn_rate_ms;
        self.arrow_anim = self.base().find_child("ThrowableArrow").and_then(|ta| {
            ta.find_child("ArrowAnim").and_then(|aa| {
                aa.get_child(0)
                    .and_then(|at| at.try_cast::<AnimationTree>().ok())
            })
        });
    }
    fn process(&mut self, delta: f64) {
        let time = Time::singleton();
        let current_time_ms = time.get_ticks_msec();
        let time_diff = self.next_spawn_time_ms.saturating_sub(current_time_ms);

        if time_diff < self.ms_before_arrow_direction_show {
            if let Some(ref mut anim) = self.arrow_anim {
                anim.set("parameters/conditions/idle", &false.to_variant());
                match self.next_arrow_position {
                    ThrowableSpawnDirection::Left => {
                        anim.set("parameters/conditions/left", &true.to_variant());
                    }
                    ThrowableSpawnDirection::Right => {
                        anim.set("parameters/conditions/right", &true.to_variant());
                    }
                    _ => {
                        anim.set("parameters/conditions/up", &true.to_variant());
                    }
                }
            }
        } else {
            if let Some(ref mut anim) = self.arrow_anim {
                anim.set("parameters/conditions/idle", &true.to_variant());
                anim.set("parameters/conditions/left", &false.to_variant());
                anim.set("parameters/conditions/right", &false.to_variant());
                anim.set("parameters/conditions/up", &false.to_variant());
            }
        }
        if current_time_ms >= self.next_spawn_time_ms {
            self.next_spawn_time_ms = current_time_ms + self.spawn_rate_ms;
            self.spawn_throwable();
        }
    }
}
#[godot_api]
impl ThrowableSpawner {}

impl ThrowableSpawner {
    fn decide_next_throw_direction() -> ThrowableSpawnDirection {
        let mut rand = RandomNumberGenerator::new_gd();
        let direction = rand.randi_range(0, 2);
        match direction {
            0 => ThrowableSpawnDirection::Up,
            1 => ThrowableSpawnDirection::Left,
            _ => ThrowableSpawnDirection::Right,
        }
    }
    fn spawn_throwable(&mut self) {
        let scene = load::<PackedScene>("res://Prefabs/throwable.tscn");
        if let Some(mut throwable) = scene
            .instantiate()
            .and_then(|log| log.try_cast::<Throwable>().ok())
        {
            throwable.set_position(self.base().get_position());
            let force_vector = Vector3 {
                x: 0.0,
                y: self.throw_force,
                z: {
                    match self.next_arrow_position {
                        ThrowableSpawnDirection::Right => -self.throw_force / 1.5,
                        ThrowableSpawnDirection::Left => self.throw_force / 1.5,
                        _ => 0.0,
                    }
                },
            };
            let mut rand = RandomNumberGenerator::new_gd();
            let type_of_throwable = rand.randi_range(0, 2);
            match type_of_throwable {
                0 => throwable.bind_mut().become_throwable(Box::new(IceChunk {})),
                1 => throwable.bind_mut().become_throwable(Box::new(Log {})),
                _ => throwable.bind_mut().become_throwable(Box::new(Salmon {})),
            }
            if let Some(mut root) = self.base().get_tree().get_current_scene() {
                root.add_child(&throwable);
            }
            throwable.apply_central_impulse(force_vector);
            self.next_arrow_position = Self::decide_next_throw_direction();
        }
    }
}
