use godot::{
    classes::{BoxContainer, IPanel, Panel},
    prelude::*,
};

use crate::{
    game_manager::game_manager::GameManager,
    ui::{label_counter::LabelCounter, player_controls_panel::PlayerControlsPanel},
};

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct PregamePanel {
    base: Base<Panel>,
    #[export]
    player_controls_panel_grid: Option<Gd<BoxContainer>>,
    player_controls_stack: Vec<Gd<PlayerControlsPanel>>,
    #[export]
    player_num_counter_label: Option<Gd<LabelCounter>>,
    #[export]
    match_time_counter_label: Option<Gd<LabelCounter>>,
    #[export]
    rounds_to_win_counter_label: Option<Gd<LabelCounter>>,

    current_amount_of_player_controls: usize,
}

#[godot_api]
impl IPanel for PregamePanel {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            player_controls_panel_grid: None,
            player_controls_stack: Vec::new(),
            current_amount_of_player_controls: 0,
            player_num_counter_label: None,
            match_time_counter_label: None,
            rounds_to_win_counter_label: None,
        }
    }
    fn ready(&mut self) {}
    fn process(&mut self, _delta: f32) {
        while let Some(ref mut player_num) = self.player_num_counter_label
            && self.current_amount_of_player_controls < player_num.bind().get_value() as usize
        {
            let player_control = load::<PackedScene>("res://Prefabs/Ui/player_control_panel.tscn");
            if let Some(mut player_control) = player_control
                .instantiate()
                .and_then(|pc| pc.try_cast::<PlayerControlsPanel>().ok())
            {
                if let Some(ref mut grid) = self.player_controls_panel_grid {
                    grid.add_child(&player_control);
                }
                player_control
                    .bind_mut()
                    .assign_player(self.current_amount_of_player_controls as u8 + 1);
                self.player_controls_stack.push(player_control);
            }
            self.current_amount_of_player_controls += 1;
        }
        while let Some(ref mut player_num) = self.player_num_counter_label
            && self.current_amount_of_player_controls > player_num.bind().get_value() as usize
        {
            let player_control_to_remove_pop = self.player_controls_stack.pop();
            if let Some(mut player_control_to_remove) = player_control_to_remove_pop {
                player_control_to_remove.call_deferred("queue_free", &[]);
            }
            self.current_amount_of_player_controls -= 1;
        }
    }
}

#[godot_api]
impl PregamePanel {
    pub fn start_game(&mut self) -> bool {
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManager");
        if self
            .player_controls_stack
            .iter()
            .all(|f| f.bind().all_controls_assigned())
            && let Some(ref mut match_time) = self.match_time_counter_label
            && let Some(ref mut player_num) = self.player_num_counter_label
            && let Some(ref mut rounds) = self.rounds_to_win_counter_label
            && let Some(mut gm) = gm
        {
            gm.bind_mut().create_game(
                player_num.bind().get_value() as u8,
                rounds.bind().get_value() as u32,
                match_time.bind().get_value() as u32,
            );
            return true;
        } else {
            return false;
        }
    }
}
