use godot::{
    classes::{GridContainer, IPanel, Panel},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Panel)]
pub struct PregamePanel {
    base: Base<Panel>,
    player_controls_pane: Option<Gd<GridContainer>>,
}

#[godot_api]
impl IPanel for PregamePanel {
    fn init(base: Base<Panel>) -> Self {
        Self {
            base,
            player_controls_pane: None,
        }
    }
    fn ready(&mut self) {}
}

#[godot_api]
impl PregamePanel {}
