use godot::{
    classes::{AnimationTree, INode3D, Label3D},
    prelude::*,
};

use crate::game_manager::game_manager::GameManager;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct RoundManager {
    base: Base<Node3D>,
    output_text: Option<Gd<Label3D>>,
    output_text_anim: Option<Gd<AnimationTree>>,
    starting_seq_started: bool,
}

#[godot_api]
impl INode3D for RoundManager {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            output_text: None,
            output_text_anim: None,
            starting_seq_started: false,
        }
    }

    fn ready(&mut self) {
        self.ready_label();
    }
    fn process(&mut self, _delta: f32) {
        if !self.starting_seq_started {
            self.starting_seq_started = true;
            let mut gm = self
                .base()
                .try_get_node_as::<GameManager>("/root/GameManager");
            if let Some(ref mut label) = self.output_text {
                label.set_visible(true);
                if let Some(ref mut gm) = gm {
                    let round = gm.bind().get_current_round_number_one_based();
                    if round > 0 {
                        label.set_text(&format!("ROUND {}!", round));
                    }
                }
            }
            let this = self.to_gd();
            let _guard = self.base_mut();
            godot::task::spawn(Self::starting_seq_routine(this));
        }
    }
}

#[godot_api]
impl RoundManager {
    #[signal]
    pub fn round_start();

    fn ready_label(&mut self) {
        self.output_text = self
            .base()
            .find_child("RoundLabel")
            .and_then(|lt| lt.try_cast::<Label3D>().ok());
        self.output_text_anim = self.base().find_child("RoundAnim").and_then(|ra| {
            ra.get_child(0)
                .and_then(|tree| tree.try_cast::<AnimationTree>().ok())
        });
    }
    async fn starting_seq_routine(mut this: Gd<Self>) {
        let mut timer;
        {
            let bind = this.bind_mut();
            timer = bind.base().get_tree().create_timer(1.0);
        }
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;
        {
            let mut bind = this.bind_mut();
            if let Some(ref mut label) = bind.output_text {
                label.set_visible(false);
            }
            if let Some(ref mut anim) = bind.output_text_anim {
                anim.set("parameters/conditions/count", &true.to_variant());
            }
            timer = bind.base().get_tree().create_timer(3.0);
        }
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;
        {
            let mut bind = this.bind_mut();
            bind.signals().round_start().emit();
        }
    }
}
