use godot::{
    classes::{Button, Control, IControl, Label, RichTextLabel},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct LabelCounter {
    base: Base<Control>,
    inc_button: Option<Gd<Button>>,
    dec_button: Option<Gd<Button>>,
    label: Option<Gd<RichTextLabel>>,
    #[export]
    possible_values: PackedArray<f32>,
    current_selection_pos: usize,
}

#[godot_api]
impl IControl for LabelCounter {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            inc_button: None,
            dec_button: None,
            label: None,
            possible_values: PackedArray::new(),
            current_selection_pos: 0,
        }
    }
    fn ready(&mut self) {
        self.dec_button = self
            .base()
            .find_child("DecButton")
            .and_then(|button| button.try_cast::<Button>().ok());
        self.inc_button = self
            .base()
            .find_child("IncButton")
            .and_then(|button| button.try_cast::<Button>().ok());
        self.label = self
            .base()
            .find_child("Label")
            .and_then(|button| button.try_cast::<RichTextLabel>().ok());
        let this = self.to_gd();
        if let Some(ref mut label) = self.label {
            label.set_text(&format!("{}", self.possible_values.get(0).unwrap_or(0.0)));
        }
        if let Some(ref mut inc) = self.inc_button
            && let Some(ref mut dec) = self.dec_button
        {
            inc.signals()
                .button_down()
                .connect_other(&this, Self::on_inc);
            dec.signals()
                .button_down()
                .connect_other(&this, Self::on_dec);
        }
    }
}

#[godot_api]
impl LabelCounter {
    fn update_label(&mut self) {
        if let Some(ref mut label) = self.label {
            label.set_text(&format!(
                "{}",
                self.possible_values
                    .get(self.current_selection_pos)
                    .unwrap_or(0.0)
            ));
        }
    }
    fn on_inc(&mut self) {
        if self.current_selection_pos < (self.possible_values.len() - 1) {
            self.current_selection_pos += 1;
            self.update_label();
        }
    }
    fn on_dec(&mut self) {
        if self.current_selection_pos > 0 {
            self.current_selection_pos -= 1;
            self.update_label();
        }
    }
}
