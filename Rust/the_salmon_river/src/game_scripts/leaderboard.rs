use godot::prelude::*;

use crate::{
    game_managers::game_manager::GameManager,
    game_scripts::game_utils::player_color_based_on_number,
};

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct LeaderBoard {
    base: Base<Node3D>,
    points_text: Option<Gd<Node3D>>,
    winner_text: Option<Gd<Node3D>>,
}

#[godot_api]
impl INode3D for LeaderBoard {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            points_text: None,
            winner_text: None,
        }
    }
    fn ready(&mut self) {
        self.points_text = self
            .base()
            .find_child("PointsText")
            .and_then(|pt| pt.try_cast::<Node3D>().ok());
        self.winner_text = self
            .base()
            .find_child("WinnerText")
            .and_then(|pt| pt.try_cast::<Node3D>().ok());
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManagerGlobal");
        if let Some(mut gm) = gm
            && let Some(ref mut points_text) = self.points_text
            && let Some(ref mut winner_text) = self.winner_text
        {
            let player_count = gm.bind_mut().get_player_count();
            let mut points_text_to_set = String::new();
            for player_num in 1..=player_count as u8 {
                let score = gm.bind().get_player_score(player_num);
                let color = player_color_based_on_number(player_num).to_html();
                points_text_to_set.push_str(&format!(
                    "[color=#{}]PLAYER {}:{}[/color] ",
                    color, player_num, score
                ));
            }
            points_text.set("text", &points_text_to_set.to_variant());
            let winner = gm.bind().get_winner();
            let color = player_color_based_on_number(winner).to_html();
            winner_text.set(
                "text",
                &format!("[color=#{}]PLAYER\n{}\nWON![/color]", color, winner).to_variant(),
            );
        }
    }
}

#[godot_api]
impl LeaderBoard {}
