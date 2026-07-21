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
    #[export]
    jump_key: Key,
    #[export]
    left_key: Key,
    #[export]
    right_key: Key,
}
#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            speed: 2.0,
            jump_force: 5.0,
            jumped: false,
            jump_key: Key::W,
            left_key: Key::A,
            right_key: Key::D,
        }
    }
    fn ready(&mut self) {}
    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut velocity = self.base().get_velocity();

        // Apply gravity
        velocity.y -= 20.0 * delta as f32;

        velocity.z = 0.0;
        if input.is_key_pressed(self.left_key) {
            velocity.z += self.speed;
        }
        if input.is_key_pressed(self.right_key) {
            velocity.z += -self.speed;
        }
        if input.is_key_pressed(self.jump_key) && self.base().is_on_floor() && !self.jumped {
            self.jumped = true;
            velocity.y = self.jump_force;
        }
        if !input.is_key_pressed(self.jump_key) {
            self.jumped = false;
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}
