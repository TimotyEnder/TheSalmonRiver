use godot::{classes::INode, prelude::*};

use crate::game_manager::game_manager::GameManager;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct LeaderBoard {
    base: Base<Node>,
    points_text: Option<Gd<Node3D>>,
    winner_text: Option<Gd<Node3D>>,
}

#[godot_api]
impl INode for LeaderBoard {
    fn init(base: Base<Node>) -> Self {
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
            .try_get_node_as::<GameManager>("/root/GameManager");
    }
}

#[godot_api]
impl LeaderBoard {}
