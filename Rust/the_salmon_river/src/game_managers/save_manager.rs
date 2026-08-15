use godot::{
    classes::{FileAccess, IRefCounted, Json, file_access::ModeFlags},
    prelude::*,
};

use crate::game_managers::player_control_scheme::PlayerControlScheme;

const SAVE_PATH: &str = "user://control_schemes.json";
#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct SaveManager {}

#[godot_api]
impl IRefCounted for SaveManager {
    fn init(_base: Base<RefCounted>) -> Self {
        Self {}
    }
}

#[godot_api]
impl SaveManager {
    pub fn save_control_schemes(&mut self, control_schemes: Vec<Gd<PlayerControlScheme>>) {
        let mut all = Dictionary::<u8, Variant>::new();
        for (i, scheme) in control_schemes.iter().enumerate() {
            all.set(i as u8 + 1, &scheme.bind().to_dict().to_variant());
        }
        if let Some(mut file) = FileAccess::open(SAVE_PATH, ModeFlags::WRITE) {
            file.store_string(&Json::stringify(&all.to_variant()));
        }
        godot_print!("saved controlschemes to {SAVE_PATH}");
    }
    pub fn load_control_schemes_from_file(&mut self) -> Vec<Gd<PlayerControlScheme>> {
        let mut to_ret = Vec::new();
        let Some(file) = FileAccess::open(SAVE_PATH, ModeFlags::READ) else {
            return to_ret;
        };
        let text = file.get_as_text();
        let all = Json::parse_string(&text)
            .try_to::<Dictionary<Variant, Variant>>()
            .unwrap_or_default();
        for player_num in 1..=all.len() {
            let key = GString::from(player_num.to_string().as_str());
            if let Some(scheme_var) = all.get(&key)
                && let Ok(scheme_dict) = scheme_var.try_to::<Dictionary<Variant, Variant>>()
            {
                to_ret.push(Gd::from_object(PlayerControlScheme::from_dict(
                    &scheme_dict,
                )));
            }
        }
        godot_print!("controlschemes loaded from {SAVE_PATH}");
        return to_ret;
    }
}
