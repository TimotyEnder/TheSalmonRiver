use godot::{
    classes::{Button, Control, IControl},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct Focuser {
    base: Base<Control>,
    #[export]
    to_focus: Option<Gd<Button>>,
}

#[godot_api]
impl IControl for Focuser {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            to_focus: None,
        }
    }
    fn ready(&mut self) {
        if let Some(ref mut focus) = self.to_focus {
            focus.grab_focus();
        }
    }
}

#[godot_api]
impl Focuser {}
