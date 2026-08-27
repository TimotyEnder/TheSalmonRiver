use godot::{
    classes::{Area3D, IArea3D, Time},
    prelude::*,
};

use crate::{
    game_managers::game_manager::GameManager,
    game_scripts::{game_utils::player_color_based_on_number, player::Player},
};

#[derive(GodotClass)]
#[class(base=Area3D)]
pub struct VotingPressurePlate {
    base: Base<Area3D>,
    players_entered: Vec<bool>,
    label: Option<Gd<Node3D>>,
    #[export]
    plate_success_condition_label: StringName,
    timer_started: bool,
    till_condition: u64,
    #[export]
    condition_timer_ms: u32,
}

#[godot_api]
impl IArea3D for VotingPressurePlate {
    fn init(base: Base<Area3D>) -> Self {
        Self {
            base,
            players_entered: Vec::new(),
            label: None,
            plate_success_condition_label: StringName::from("Condition"),
            timer_started: false,
            till_condition: 0,
            condition_timer_ms: 5000,
        }
    }
    fn ready(&mut self) {
        self.label = self
            .base()
            .find_child("SignLabel")
            .and_then(|label| label.try_cast::<Node3D>().ok());
        let this = self.to_gd();
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
        if let Some(mut gm) = gm {
            let player_count = gm.bind_mut().get_player_count();
            self.players_entered = vec![false; player_count as usize];
        }
        self.signals()
            .area_entered()
            .connect_other(&this, Self::on_body_entered_on_pressure_plate);
        self.signals()
            .area_exited()
            .connect_other(&this, Self::on_body_exited_from_pressure_plate);
    }
    fn process(&mut self, _delta: f32) {
        let count = self.majority_standing_on_plate();
        let time = Time::singleton();
        let mut seconds_left_string = String::from("");
        if !self.timer_started {
            self.till_condition = time.get_ticks_msec() + self.condition_timer_ms as u64;
            self.timer_started = true;
        }
        if count {
            let msecs_left = time.get_ticks_msec().abs_diff(self.till_condition);
            let secs_left = msecs_left / 1000;
            if secs_left <= 0 {
                self.signals().pressure_plate_condition_met().emit();
            }
            seconds_left_string = format!("{}S", secs_left);
        } else {
            self.timer_started = false;
        }
        self.update_label_text(seconds_left_string);
    }
}

#[godot_api]
impl VotingPressurePlate {
    #[signal]
    pub fn pressure_plate_condition_met();
    fn majority_standing_on_plate(&mut self) -> bool {
        return self
            .players_entered
            .iter()
            .filter(|f| **f == false)
            .collect::<Vec<&bool>>()
            .len()
            < self
                .players_entered
                .iter()
                .filter(|f| **f == true)
                .collect::<Vec<&bool>>()
                .len();
    }
    fn on_body_entered_on_pressure_plate(&mut self, area: Gd<Area3D>) {
        if area.get_name().contains("Player") {
            let player_entered = area
                .get_parent()
                .and_then(|player| player.try_cast::<Player>().ok());
            if let Some(player) = player_entered {
                let player_num = player.bind().get_player_num();
                self.players_entered[player_num as usize - 1] = true;
            }
        }
    }
    fn on_body_exited_from_pressure_plate(&mut self, area: Gd<Area3D>) {
        if area.get_name().contains("Player") {
            let player_exiting = area
                .get_parent()
                .and_then(|player| player.try_cast::<Player>().ok());
            if let Some(player) = player_exiting {
                let player_num = player.bind().get_player_num();
                self.players_entered[player_num as usize - 1] = false;
            }
        }
    }
    fn update_label_text(&mut self, seconds_left_string: String) {
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");

        if let Some(ref mut label) = self.label
            && let Some(gm) = gm
        {
            let mut text_to_set = self.plate_success_condition_label.to_string() + " ";
            text_to_set += &seconds_left_string;
            text_to_set += "\n";
            for player_num in 1..=self.players_entered.len() {
                if let Some(team) = gm.bind().get_player_team_num(player_num as u8) {
                    text_to_set += &format!(
                        "|[color=#{}]P{}[/color]|",
                        {
                            if self.players_entered[player_num - 1] {
                                player_color_based_on_number(team).to_html()
                            } else {
                                Color::WHITE.to_html()
                            }
                        },
                        player_num
                    );
                }
            }
            label.set("text", &text_to_set.to_variant());
        }
    }
}
