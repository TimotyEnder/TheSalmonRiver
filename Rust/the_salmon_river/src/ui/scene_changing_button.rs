use godot::{
    classes::{Button, IButton},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct SceneChangingButton {
    base: Base<Button>,
    #[export]
    scene_name_to_change_to: StringName,
}

#[godot_api]
impl IButton for SceneChangingButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            scene_name_to_change_to: StringName::from(""),
        }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        self.signals()
            .button_down()
            .connect_other(&this, Self::scene_change);
    }
}

#[godot_api]
impl SceneChangingButton {
    fn scene_change(&mut self) {
        let scene_name = format!("res://Scenes/{}.tscn", self.scene_name_to_change_to);
        self.base().get_tree().change_scene_to_file(&scene_name);
    }
}
