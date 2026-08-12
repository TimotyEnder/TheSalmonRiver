use godot::{
    classes::{IPanel, Panel},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct PregamePanel {
    base: Base<Panel>,
}

#[godot_api]
impl IPanel for PregamePanel {
    fn init(base: Base<Panel>) -> Self {
        Self { base }
    }
    fn ready(&mut self) {}
}

#[godot_api]
impl PregamePanel {}
