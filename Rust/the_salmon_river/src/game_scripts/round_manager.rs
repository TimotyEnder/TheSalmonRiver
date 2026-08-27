use godot::{
    classes::{AnimationTree, INode3D, Node3D, Time},
    prelude::*,
};

use crate::game_scripts::{
    game_utils::player_color_based_on_number, temporary_obsticle_manager::TemporaryObsticleManager,
};
use crate::{
    game_managers::{
        audio_manager::{AudioManager, AudioPlayBuilder},
        game_manager::GameManager,
    },
    game_scripts::game_utils::vec3_to_vec2,
};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct RoundManager {
    base: Base<Node3D>,
    output_text: Option<Gd<Node3D>>,
    output_text_anim: Option<Gd<AnimationTree>>,
    starting_seq_started: bool,
    players_left: u32,
    players_teams_alive: Vec<Vec<u8>>,
    run_round_timer: bool,
    timer_time: u64,
    timer_flag: bool,
    next_temp_obsticle_spawn_ms: u64,
    temp_obsticle_spawn_rate_ms: u64,
    player_health_vec: Vec<i8>,
    #[export]
    temp_obsticle_manager: Option<Gd<TemporaryObsticleManager>>,
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
            players_teams_alive: Vec::new(),
            run_round_timer: false,
            timer_flag: false,
            timer_time: 0,
            player_health_vec: Vec::new(),
            temp_obsticle_manager: None,
            next_temp_obsticle_spawn_ms: 0,
            temp_obsticle_spawn_rate_ms: 20000,
        }
    }

    fn ready(&mut self) {
        self.ready_label();
        self.ready_from_gamemanager();
    }
    fn process(&mut self, _delta: f32) {
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if !self.starting_seq_started {
            self.starting_seq_started = true;
            let mut gm = self
                .base()
                .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
            if let Some(ref mut label) = self.output_text {
                label.set_visible(true);

                if let Some(ref mut gm) = gm
                    && let Some(mut audio) = audio
                {
                    let round = gm.bind().get_current_round_number_one_based();
                    if round > 0 {
                        let mut text = format!("ROUND {}!\n", round);
                        for team_num in 1..=4 as u8 {
                            if !self.players_teams_alive[team_num as usize - 1].is_empty() {
                                let score = gm.bind().get_team_score(team_num);
                                let color = player_color_based_on_number(team_num).to_html();
                                text.push_str(&format!("[color=#{}]{}[/color] ", color, score));
                            }
                        }
                        label.set("text", &text.to_variant());
                        audio.bind_mut().play_sound_built(
                            AudioPlayBuilder::play_sound_effect(
                                crate::sound_utils::SoundEffect::MatchStartJingle,
                            )
                            .at_position(vec3_to_vec2(self.base().get_global_position())),
                        );
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
                if self.next_temp_obsticle_spawn_ms < time.get_ticks_msec()
                    && let Some(ref mut temp_obs) = self.temp_obsticle_manager
                {
                    self.next_temp_obsticle_spawn_ms =
                        time.get_ticks_msec() + self.temp_obsticle_spawn_rate_ms;
                    temp_obs.bind_mut().spawn_temp_obsticle();
                }
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

    #[func]
    pub fn play_coundown_tick(&mut self) {
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        if let Some(mut audio) = audio {
            audio.bind_mut().play_sound_built(
                AudioPlayBuilder::play_sound_effect(crate::sound_utils::SoundEffect::CountdownTick)
                    .at_position(vec3_to_vec2(self.base().get_global_position())),
            );
        }
    }
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
            self.players_teams_alive = vec![
                Vec::<u8>::new(),
                Vec::<u8>::new(),
                Vec::<u8>::new(),
                Vec::<u8>::new(),
            ];
            self.players_left = gm.bind_mut().get_player_count();
            (1..=self.players_left as u8).for_each(|f| {
                if let Some(team_num) = gm.bind().get_player_team_num(f) {
                    self.players_teams_alive[team_num as usize - 1].push(f)
                }
            });
            self.player_health_vec = vec![-1; self.players_left as usize];
        }
    }
    pub fn report_player_death(&mut self, player_num: u8) {
        for team in self.players_teams_alive.iter_mut() {
            let posible_position_to_remove = team.iter().position(|element| *element == player_num);
            if let Some(pos) = posible_position_to_remove {
                team.remove(pos);
            }
        }
        self.players_left = self.players_left.saturating_sub(1);
        self.player_health_vec[player_num as usize - 1] = 0;
        if !self.more_than_one_team_remaining() {
            if let Some((winner, _)) = self
                .players_teams_alive
                .iter()
                .enumerate()
                .filter(|(_, team)| !team.is_empty())
                .next()
            {
                self.make_team_win(winner as u8 + 1);
            }
        }
    }
    pub fn more_than_one_team_remaining(&self) -> bool {
        return self
            .players_teams_alive
            .iter()
            .filter(|team| team.len() > 0)
            .count()
            > 1;
    }
    pub fn report_player_health(&mut self, health: u8, player_num: u8) {
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");

        self.player_health_vec[player_num as usize - 1] = health as i8;
        if !self.player_health_vec.contains(&-1i8)
            && let Some(mut gm) = gm
        {
            let mut team_total_hp = vec![0; 4];
            let mut players_per_team = vec![0; 4];
            let mut index = 1;
            for player in self.player_health_vec.iter() {
                if let Some(team_num) = gm.bind().get_player_team_num(index) {
                    team_total_hp[team_num as usize - 1] += player;
                    players_per_team[team_num as usize - 1] += 1;
                }
                index += 1;
            }
            team_total_hp
                .iter_mut()
                .enumerate()
                .for_each(|(index, health)| *health = *health / players_per_team[index].max(1));
            let Some(max) = team_total_hp.iter().max() else {
                return;
            };
            let winning_teams = team_total_hp
                .iter()
                .copied()
                .enumerate()
                .filter(|&(_, f)| f >= *max)
                .map(|(i, _)| i as u8)
                .collect::<Vec<u8>>();
            if winning_teams.len() > 1 {
                if winning_teams.len() < gm.bind().get_team_count() as usize {
                    for team in winning_teams {
                        gm.bind_mut().team_won_round(team + 1);
                    }
                }
                self.make_tie();
            } else {
                self.make_team_win(winning_teams[0] as u8 + 1);
            }
        }
    }
    fn make_tie(&mut self) {
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        let sound_pos = self.base().get_global_position();
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
        if let Some(mut gm) = gm
            && let Some(ref mut label) = self.output_text
            && let Some(mut audio) = audio
        {
            gm.bind_mut().round_tie();
            audio.bind_mut().play_sound_built(
                AudioPlayBuilder::play_sound_effect(
                    crate::sound_utils::SoundEffect::MatchTieJingle,
                )
                .at_position(vec3_to_vec2(sound_pos)),
            );
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
    fn make_team_win(&mut self, winner: u8) {
        let audio = self
            .base()
            .try_get_node_as::<AudioManager>("/root/AudioManagerGlobal");
        let sound_pos = self.base().get_global_position();
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
        if let Some(mut gm) = gm
            && let Some(ref mut label) = self.output_text
        {
            gm.bind_mut().team_won_round(winner);
            if let Some(mut audio) = audio {
                audio.bind_mut().play_sound_built(
                    AudioPlayBuilder::play_sound_effect(
                        crate::sound_utils::SoundEffect::MatchWinnerJingle,
                    )
                    .at_position(vec3_to_vec2(sound_pos)),
                );
            }
            self.run_round_timer = false;
            label.set_visible(true);
            label.set(
                "text",
                &format!(
                    "[color=#{}]TEAM {} WINS![/color]",
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
