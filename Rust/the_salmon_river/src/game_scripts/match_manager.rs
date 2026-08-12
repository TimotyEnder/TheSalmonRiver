use godot::{classes::INode, prelude::*};

use crate::{game_manager::game_manager::GameManager, game_scripts::player::Player};

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
    fn ready(&mut self) {
        let gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManager");
        if let Some(mut gm) = gm {
            let player_count = gm.bind_mut().get_player_count();
            let player_positions = [
                &self.player_position1,
                &self.player_position2,
                &self.player_position3,
                &self.player_position4,
            ];
            for i in 0..player_count {
                let scene = load::<PackedScene>("res://Prefabs/player.tscn");
                if let Some(mut player_spawned) = scene
                    .instantiate()
                    .and_then(|ps| ps.try_cast::<Player>().ok())
                {
                    self.base()
                        .get_tree()
                        .get_current_scene()
                        .and_then(|mut scene| {
                            Some(scene.call_deferred("add_child", &[player_spawned.to_variant()]))
                        });
                    let control_scheme = gm.bind_mut().request_player_controls(i as u8 + 1);
                    player_spawned
                        .bind_mut()
                        .load_control_scheeme(control_scheme);
                    player_spawned.bind_mut().assign_player_num(i as u8 + 1);
                    if let Some(spawn_position) = player_positions[i as usize] {
                        player_spawned.set_position(spawn_position.get_position());
                    }
                }
            }
        }
    }
}

#[godot_api]
impl MatchManager {}
