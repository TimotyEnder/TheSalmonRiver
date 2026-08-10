use std::collections::HashSet;

use godot::{
    classes::{AnimationTree, INode3D, Node3D},
    prelude::*,
};

use crate::game_manager::game_manager::GameManager;
use crate::game_scripts::game_utils::player_color_based_on_number;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct RoundManager {
    base: Base<Node3D>,
    output_text: Option<Gd<Node3D>>,
    output_text_anim: Option<Gd<AnimationTree>>,
    starting_seq_started: bool,
    players_left: u32,
    players_alive: HashSet<u8>,
}

#[godot_api]
impl INode3D for RoundManager {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            output_text: None,
            output_text_anim: None,
            starting_seq_started: false,
            players_left: 0,
            players_alive: HashSet::new(),
        }
    }

    fn ready(&mut self) {
        self.ready_label();
        self.ready_from_gamemanager();
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
                        let player_count = gm.bind_mut().get_player_count();
                        let mut text = format!("ROUND {}!\n", round);
                        for player_num in 1..=player_count as u8 {
                            let score = gm.bind().get_player_score(player_num);
                            let color = player_color_based_on_number(player_num).to_html();
                            text.push_str(&format!("[color=#{}]{}[/color] ", color, score));
                        }
                        label.set("text", &text.to_variant());
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
            .and_then(|lt| lt.try_cast::<Node3D>().ok());
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
    fn ready_from_gamemanager(&mut self) {
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManager");
        if let Some(mut gm) = gm {
            self.players_left = gm.bind_mut().get_player_count();
            (1..self.players_left as u8).for_each(|f| {
                self.players_alive.insert(f);
            });
        }
    }
    pub fn report_player_death(&mut self, player_num: u8) {
        self.players_alive.remove(&player_num);
        self.players_left = self.players_left.saturating_sub(1);
        if self.players_left <= 1 {
            let mut gm = self
                .base()
                .try_get_node_as::<GameManager>("/root/GameManager");
            if let Some(ref mut gm) = gm
                && let Some(winner) = self.players_alive.iter().next()
                && let Some(ref mut label) = self.output_text
            {
                gm.bind_mut().player_won_round(*winner);
                label.set_visible(true);
                label.set(
                    "text",
                    &format!(
                        "[color=#{}]PLAYER {} WINS![/color]",
                        crate::game_scripts::game_utils::player_color_based_on_number(*winner)
                            .to_html(),
                        winner
                    )
                    .to_variant(),
                );
                let timer = self.base().get_tree().create_timer(5.0);
                let mut tree = self.base().get_tree();
                godot::task::spawn(async move {
                    Signal::from_object_signal(&timer, "timeout")
                        .to_future::<()>()
                        .await;
                    tree.change_scene_to_file("res://Scenes/main.tscn");
                });
            }
        }
    }
}
