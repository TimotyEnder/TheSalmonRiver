use std::collections::HashSet;

use godot::{
    classes::{AnimationTree, INode3D, Node3D, Time},
    prelude::*,
};

use crate::game_managers::game_manager::GameManager;
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
    run_round_timer: bool,
    timer_time: u64,
    timer_flag: bool,
    player_health_vec: Vec<i8>,
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
            run_round_timer: false,
            timer_flag: false,
            timer_time: 0,
            player_health_vec: Vec::new(),
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
                .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
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
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
        let time = Time::singleton();
        if let Some(ref mut label) = self.output_text {
            if self.run_round_timer {
                if !self.timer_flag {
                    self.timer_flag = true;

                    if let Some(gm) = gm {
                        self.timer_time = time.get_ticks_usec()
                            + gm.bind().get_round_timer_secs() as u64 * 1_000_000;
                    }
                    label.set_visible(true);
                }
                let time_remaining: f32 = ((time.get_ticks_usec() as f32 - self.timer_time as f32)
                    / 1_000_000.0)
                    .round()
                    .abs();
                label.set("text", &format!("{}", time_remaining).to_variant());
                if time_remaining <= 0.0 {
                    self.run_round_timer = false;
                    self.signals().round_timeout().emit();
                }
            }
        }
    }
}

#[godot_api]
impl RoundManager {
    #[signal]
    pub fn round_start();

    #[signal]
    pub fn round_timeout();

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
        if !this.is_instance_valid() {
            return;
        }
        let mut timer;
        {
            let bind = this.bind_mut();
            let Some(mut tree) = bind.base().get_tree_or_null() else {
                return;
            };
            timer = tree.create_timer(1.5);
        }
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;
        if !this.is_instance_valid() {
            return;
        }
        {
            let mut bind = this.bind_mut();
            if let Some(ref mut label) = bind.output_text {
                label.set_visible(false);
            }
            if let Some(ref mut anim) = bind.output_text_anim {
                anim.set("parameters/conditions/count", &true.to_variant());
            }
            let Some(mut tree) = bind.base().get_tree_or_null() else {
                return;
            };
            timer = tree.create_timer(3.0);
        }
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;
        if !this.is_instance_valid() {
            return;
        }
        {
            let mut bind = this.bind_mut();
            bind.signals().round_start().emit();
            let Some(mut tree) = bind.base().get_tree_or_null() else {
                return;
            };
            timer = tree.create_timer(1.0);
        }
        Signal::from_object_signal(&timer, "timeout")
            .to_future::<()>()
            .await;
        if !this.is_instance_valid() {
            return;
        }
        {
            let mut bind = this.bind_mut();
            bind.run_round_timer = true;
        }
    }
    fn ready_from_gamemanager(&mut self) {
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
        if let Some(mut gm) = gm {
            self.players_left = gm.bind_mut().get_player_count();
            (1..=self.players_left as u8).for_each(|f| {
                self.players_alive.insert(f);
            });
            self.player_health_vec = vec![-1; self.players_left as usize];
        }
    }
    pub fn report_player_death(&mut self, player_num: u8) {
        self.players_alive.remove(&player_num);
        self.players_left = self.players_left.saturating_sub(1);
        self.player_health_vec[player_num as usize - 1] = 0;
        if self.players_left <= 1 {
            if let Some(winner) = self.players_alive.iter().next() {
                self.make_player_win(*winner);
            }
        }
    }
    pub fn player_health_report(&mut self, health: u8, player_num: u8) {
        self.player_health_vec[player_num as usize - 1] = health as i8;
        if !self.player_health_vec.contains(&-1i8) {
            let max = self
                .player_health_vec
                .iter()
                .fold(i8::MIN, |a, b| a.max(*b));
            let winners = self
                .player_health_vec
                .iter()
                .copied()
                .enumerate()
                .filter(|&(_, f)| f >= max)
                .map(|(i, _)| i as u8)
                .collect::<Vec<u8>>();
            if winners.len() > 1 {
                self.make_tie();
            } else {
                self.make_player_win(winners[0] as u8 + 1);
            }
        }
    }
    fn make_tie(&mut self) {
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
        if let Some(mut gm) = gm
            && let Some(ref mut label) = self.output_text
        {
            gm.bind_mut().round_tie();
            self.run_round_timer = false;
            label.set_visible(true);
            label.set("text", &format!("TIE!").to_variant());
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
    fn make_player_win(&mut self, winner: u8) {
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
        if let Some(mut gm) = gm
            && let Some(ref mut label) = self.output_text
        {
            gm.bind_mut().player_won_round(winner);
            self.run_round_timer = false;
            label.set_visible(true);
            label.set(
                "text",
                &format!(
                    "[color=#{}]PLAYER {} WINS![/color]",
                    crate::game_scripts::game_utils::player_color_based_on_number(winner).to_html(),
                    winner
                )
                .to_variant(),
            );
            let timer = self.base().get_tree().create_timer(5.0);
            let mut this = self.to_gd();
            godot::task::spawn(async move {
                Signal::from_object_signal(&timer, "timeout")
                    .to_future::<()>()
                    .await;
                if this.is_instance_valid() {
                    this.bind_mut().decide_what_scene_to_show();
                }
            });
        }
    }
    fn decide_what_scene_to_show(&mut self) {
        let mut tree = self.base().get_tree();
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
        if let Some(gm) = gm {
            if gm.bind().is_game_finished() {
                tree.change_scene_to_file("res://Scenes/victory_screen.tscn");
            } else {
                tree.change_scene_to_file("res://Scenes/main.tscn");
            }
        }
    }
}
