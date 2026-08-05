use crate::{
    throwable::Throwable,
    throwables::{ice_chunk::IceChunk, log::Log, salmon::Salmon},
};
use godot::{
    classes::{RandomNumberGenerator, Time},
    prelude::*,
};
#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ThrowableSpawner {
    base: Base<Node3D>,
    throw_force: f32,
    next_spawn_time_ms: u64,
    spawn_rate_ms: u64,
}
#[godot_api]
impl INode3D for ThrowableSpawner {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base: base,
            throw_force: 5.0,
            next_spawn_time_ms: 0,
            spawn_rate_ms: 4000,
        }
    }
    fn ready(&mut self) {
        self.next_spawn_time_ms = self.spawn_rate_ms;
    }
    fn process(&mut self, delta: f64) {
        let time = Time::singleton();
        let current_time_ms = time.get_ticks_msec();
        if current_time_ms >= self.next_spawn_time_ms {
            self.next_spawn_time_ms = current_time_ms + self.spawn_rate_ms;
            self.spawn_throwable();
        }
    }
}
#[godot_api]
impl ThrowableSpawner {}

impl ThrowableSpawner {
    fn spawn_throwable(&self) {
        let scene = load::<PackedScene>("res://Prefabs/throwable.tscn");
        if let Some(mut throwable) = scene
            .instantiate()
            .and_then(|log| log.try_cast::<Throwable>().ok())
        {
            throwable.set_position(self.base().get_position());
            let mut rand = RandomNumberGenerator::new_gd();
            let direction = rand.randi_range(0, 1);
            let force_vector = Vector3 {
                x: 0.0,
                y: self.throw_force,
                z: {
                    match direction {
                        0 => -self.throw_force / 2.0,
                        _ => self.throw_force / 2.0,
                    }
                },
            };
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
        }
    }
}
