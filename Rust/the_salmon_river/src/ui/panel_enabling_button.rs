use godot::{
    classes::{Button, IButton, Panel},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct PanelEnablingButton {
    base: Base<Button>,
    #[export]
    panel_to_enable: Option<Gd<Panel>>,
    #[export]
    disable_panel: bool,
}

#[godot_api]
impl IButton for PanelEnablingButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
            panel_to_enable: None,
            disable_panel: false,
        }
    }
    fn ready(&mut self) {
        let this = self.to_gd();
        self.base()
            .signals()
            .button_down()
            .connect_other(&this, Self::on_press);
    }
}

#[godot_api]
impl PanelEnablingButton {
    #[func]
    fn on_press(&mut self) {
        if let Some(ref mut controls_panel) = self.panel_to_enable {
            if self.disable_panel {
                controls_panel.set_visible(false);
            } else {
                controls_panel.set_visible(true);
            }
        }
    }
}
