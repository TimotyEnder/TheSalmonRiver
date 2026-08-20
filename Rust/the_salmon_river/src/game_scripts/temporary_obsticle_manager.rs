use godot::{
    classes::{AnimationPlayer, AnimationTree, INode3D, RandomNumberGenerator},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct TemporaryObsticleManager {
    base: Base<Node3D>,
    animation_tree: Option<Gd<AnimationTree>>,
    animation_triggers: Vec<&'static str>,
    last_chosen_index: usize,
}

#[godot_api]
impl INode3D for TemporaryObsticleManager {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            animation_tree: None,
            animation_triggers: vec!["single_platform_center", "double_platform"],
            last_chosen_index: 1,
        }
    }
    fn ready(&mut self) {
        self.animation_tree = self
            .base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer")
            .get_child(0)
            .and_then(|tree| tree.try_cast::<AnimationTree>().ok());
    }
}

#[godot_api]
impl TemporaryObsticleManager {
    #[func]
    pub fn spawn_temp_obsticle(&mut self) {
        let mut rand = RandomNumberGenerator::new_gd();
        if let Some(ref mut tree) = self.animation_tree {
            tree.set("parameters/conditions/exit", &true.to_variant());
            (0..self.animation_triggers.len()).for_each(|index| {
                tree.set(
                    &format!("parameters/conditions/{}", self.animation_triggers[index]),
                    &false.to_variant(),
                );
                let debug = format!("parameters/conditions/{}", self.animation_triggers[index]);
                godot_print!("{debug}");
            });
            let mut tree = tree.clone();
            let timer = self.base().get_tree().create_timer(2.0);
            let mut this = self.to_gd();
            godot::task::spawn(async move {
                Signal::from_object_signal(&timer, "timeout")
                    .to_future::<()>()
                    .await;
                let mut animation_trigger_chosen = rand
                    .randf_range(0.0, this.bind().animation_triggers.len() as f32 - 1.0)
                    .round() as usize;
                while animation_trigger_chosen == this.bind().last_chosen_index {
                    animation_trigger_chosen = rand
                        .randf_range(0.0, this.bind().animation_triggers.len() as f32 - 1.0)
                        .round() as usize;
                }
                this.bind_mut().last_chosen_index = animation_trigger_chosen;
                let trigger_str = format!(
                    "parameters/conditions/{}",
                    this.bind_mut().animation_triggers[animation_trigger_chosen]
                );
                godot_print!("{trigger_str}");
                tree.set("parameters/conditions/exit", &false.to_variant());
                tree.set(&trigger_str, &true.to_variant());
            });
        }
    }
}
