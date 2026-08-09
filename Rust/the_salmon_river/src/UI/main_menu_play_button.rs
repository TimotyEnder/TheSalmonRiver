use godot::{
    classes::{Button, IButton},
    prelude::*,
};
#[derive(GodotClass)]
#[class(base=Button)]

pub struct MainMenuPlayButton {
    base: Base<Button>,
}
#[godot_api]
impl IButton for MainMenuPlayButton {
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
impl MainMenuPlayButton {
    #[func]
    fn on_pressed(&mut self) {
        let mut tree = self.base().get_tree();
        tree.change_scene_to_file("res://Scenes/main.tscn");
    }
}
