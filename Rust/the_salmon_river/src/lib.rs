pub mod game_manager;
pub mod game_scripts;
pub mod throwables;
pub mod ui;
use godot::prelude::*;

struct TheSalmonRiver;

#[gdextension]
unsafe impl ExtensionLibrary for TheSalmonRiver {}
