use godot::{
    classes::{BoxContainer, IPanel, Panel},
    prelude::*,
};

use crate::ui::{label_counter::LabelCounter, player_controls_panel::PlayerControlsPanel};

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct PregamePanel {
    base: Base<Panel>,
    #[export]
    player_controls_panel_grid: Option<Gd<BoxContainer>>,
    #[export]
    player_num_counter_label: Option<Gd<LabelCounter>>,
    #[export]
    match_time_counter_label: Option<Gd<LabelCounter>>,
    current_amount_of_player_controls: usize,
}

#[godot_api]
impl IPanel for PregamePanel {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            player_controls_panel_grid: None,
            current_amount_of_player_controls: 0,
            player_num_counter_label: None,
            match_time_counter_label: None,
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
                player_control
                    .bind_mut()
                    .assing_player(self.current_amount_of_player_controls as u8 + 1);
                if let Some(ref mut grid) = self.player_controls_panel_grid {
                    grid.add_child(&player_control);
                }
            }
            self.current_amount_of_player_controls += 1;
        }
    }
}

#[godot_api]
impl PregamePanel {}
