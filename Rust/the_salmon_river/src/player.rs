use godot::classes::{CharacterBody3D, ICharacterBody3D, Input};
use godot::global::Key;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>,
    speed: f32,
    jump_force: f32,
    jumped: bool,
}
#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            speed: 2.0,
            jump_force: 5.0,
            jumped: false,
        }
    }
    fn ready(&mut self) {}
    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut velocity = self.base().get_velocity();

        // Apply gravity
        velocity.y -= 20.0 * delta as f32;

        velocity.z = 0.0;
        if input.is_key_pressed(Key::A) {
            velocity.z += self.speed;
        }
        if input.is_key_pressed(Key::D) {
            velocity.z += -self.speed;
        }
        if input.is_key_pressed(Key::W) && self.base().is_on_floor() && !self.jumped {
            self.jumped = true;
            velocity.y = self.jump_force;
        }
        if !input.is_key_pressed(Key::W) {
            self.jumped = false;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}
