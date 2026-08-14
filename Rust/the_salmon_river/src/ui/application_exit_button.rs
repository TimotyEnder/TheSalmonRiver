use godot::{
    classes::{Button, IButton},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct ApplicationExitButton {
    base: Base<Button>,
}

#[godot_api]
impl IButton for ApplicationExitButton {
    fn init(base: Base<Button>) -> Self {
        Self { base }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        self.base()
            .signals()
            .button_down()
            .connect_other(&this, Self::application_exit);
    }
}

#[godot_api]
impl ApplicationExitButton {
    fn application_exit(&mut self) {
        self.base_mut().get_tree().quit();
    }
}
