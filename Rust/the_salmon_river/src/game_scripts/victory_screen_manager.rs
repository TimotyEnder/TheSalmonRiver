use godot::{classes::INode3D, prelude::*};

use crate::{
    game_manager::game_manager::GameManager,
    game_scripts::voting_pressure_plate::VotingPressurePlate,
};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct VictoryScreenManager {
    base: Base<Node3D>,
    #[export]
    vp_rematch: Option<Gd<VotingPressurePlate>>,
    #[export]
    vp_menu: Option<Gd<VotingPressurePlate>>,
}

#[godot_api]
impl INode3D for VictoryScreenManager {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            vp_menu: None,
            vp_rematch: None,
        }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        if let Some(ref mut menu) = self.vp_menu
            && let Some(ref mut rematch) = self.vp_rematch
        {
            menu.signals()
                .pressure_plate_condition_met()
                .connect_other(&this, Self::main_menu);
            rematch
                .signals()
                .pressure_plate_condition_met()
                .connect_other(&this, Self::rematch);
        }
    }
}

#[godot_api]
impl VictoryScreenManager {
    fn rematch(&mut self) {
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManager");
        if let Some(mut gm) = gm {
            let player_count = gm.bind_mut().get_player_count();
            let match_time = gm.bind().get_round_timer_secs();
            gm.bind_mut().create_game(player_count as u8, 3, match_time);
            self.base()
                .get_tree()
                .change_scene_to_file("res://Scenes/main.tscn");
        }
    }
    fn main_menu(&mut self) {
        self.base()
            .get_tree()
            .change_scene_to_file("res://Scenes/main_menu.tscn");
    }
}
