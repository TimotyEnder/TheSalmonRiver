use godot::{
    classes::{Button, IButton},
    prelude::*,
};

use crate::game_manager::game_manager::GameManager;
#[derive(GodotClass)]
#[class(base=Button)]

pub struct MainMenuStartMatchButton {
    base: Base<Button>,
}
#[godot_api]
impl IButton for MainMenuStartMatchButton {
    fn init(base: Base<Button>) -> Self {
        Self { base }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        self.base()
            .signals()
            .pressed()
            .connect_other(&this, Self::on_pressed);
    }
}

#[godot_api]
impl MainMenuStartMatchButton {
    #[func]
    fn on_pressed(&mut self) {
        let mut tree = self.base().get_tree();
        let mut gm = self
            .base()
            .try_get_node_as::<GameManager>("/root/GameManager");
        if let Some(ref mut gm) = gm {
            gm.bind_mut().create_game(2, 3, 99);
            tree.change_scene_to_file("res://Scenes/main.tscn");
        }
    }
}
