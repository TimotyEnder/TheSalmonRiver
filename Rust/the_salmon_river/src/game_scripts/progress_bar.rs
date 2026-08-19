use godot::{
    classes::{INode3D, Sprite3D, Time},
    prelude::*,
};

use crate::game_scripts::game_utils::complementary_color;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct ProgressBar3D {
    base: Base<Node3D>,
    main_bar: Option<Gd<Node3D>>,
    diff_bar: Option<Gd<Node3D>>,
    initial_scale: Vector3,
    next_diff_bar_check_ms: u64,
    diff_bar_update_freq_ms: u64,
    ms_for_diff_bar_to_reach_bar: u64,
    #[export]
    diff_bar_enabled: bool,
}

#[godot_api]
impl INode3D for ProgressBar3D {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            diff_bar: None,
            main_bar: None,
            initial_scale: Vector3::ZERO,
            ms_for_diff_bar_to_reach_bar: 1000,
            diff_bar_update_freq_ms: 100,
            next_diff_bar_check_ms: 0,
            diff_bar_enabled: true,
        }
    }
    fn ready(&mut self) {
        self.main_bar = self
            .base()
            .find_child("MainBar")
            .and_then(|mb| mb.try_cast::<Node3D>().ok());
        self.diff_bar = self
            .base()
            .find_child("DiffBar")
            .and_then(|df| df.try_cast::<Node3D>().ok());
        if let Some(ref main) = self.main_bar {
            self.initial_scale = main.get_scale();
        }
    }
    fn process(&mut self, _delta: f32) {
        let time = Time::singleton();
        if time.get_ticks_msec() > self.next_diff_bar_check_ms && self.diff_bar_enabled {
            self.next_diff_bar_check_ms = time.get_ticks_msec() + self.diff_bar_update_freq_ms;
            self.update_diff_bar();
        }
    }
}

#[godot_api]
impl ProgressBar3D {
    fn update_diff_bar(&mut self) {
        if let Some(ref main) = self.main_bar
            && let Some(ref mut diff) = self.diff_bar
        {
            if main.get_scale().x < diff.get_scale().x {
                let mut diff_scale = diff.get_scale();
                let main_scale = main.get_scale();
                let diff_percentage = diff_scale.x / self.initial_scale.x;
                let main_percentage = main_scale.x / self.initial_scale.x;
                let percentage_diff = (main_percentage - diff_percentage).abs();
                let percentage_step = percentage_diff
                    / (self.ms_for_diff_bar_to_reach_bar as f32
                        / self.diff_bar_update_freq_ms as f32);
                let update_percentage = diff_percentage - percentage_step;
                diff_scale.x = self.initial_scale.x * update_percentage;
                diff.set_scale(diff_scale);
            } else if main.get_scale().x > diff.get_scale().x {
                let scale = main.get_scale();
                diff.set_scale(scale);
            }
        }
    }
    pub fn set_color(&mut self, color: Color) {
        let diff_color = if self.diff_bar_enabled {
            complementary_color(color)
        } else {
            Color::BLACK
        };
        if let Some(ref main) = self.main_bar
            && let Some(ref diff) = self.diff_bar
        {
            main.get_child(0).and_then(|node| {
                node.try_cast::<Sprite3D>()
                    .ok()
                    .and_then(|mut sprite| Some(sprite.set_modulate(color)))
            });
            diff.get_child(0).and_then(|node| {
                node.try_cast::<Sprite3D>()
                    .ok()
                    .and_then(|mut sprite| Some(sprite.set_modulate(diff_color)))
            });
        }
    }
    pub fn set_value_f0to1(&mut self, val: f32) {
        if let Some(ref mut main) = self.main_bar {
            let mut scale = main.get_scale();
            scale.x = self.initial_scale.x * val;
            main.set_scale(scale);
            let scale = main.get_scale();
            godot_print!("{scale}");
        }
    }
    pub fn reset(&mut self) {
        if let Some(ref mut main) = self.main_bar {
            main.set_scale(self.initial_scale);
        }
    }
}
