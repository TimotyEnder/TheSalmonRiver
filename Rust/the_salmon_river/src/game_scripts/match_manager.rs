use godot::{classes::INode, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MatchManager {
    base: Base<Node>,
    #[export]
    player_position1: Option<Gd<Node3D>>,
    #[export]
    player_position2: Option<Gd<Node3D>>,
    #[export]
    player_position3: Option<Gd<Node3D>>,
    #[export]
    player_position4: Option<Gd<Node3D>>,
}

#[godot_api]
impl INode for MatchManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            player_position1: None,
            player_position2: None,
            player_position3: None,
            player_position4: None,
        }
    }
    fn ready(&mut self) {}
}

#[godot_api]
impl MatchManager {}
