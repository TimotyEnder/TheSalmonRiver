pub mod game_managers;
pub mod game_scripts;
pub mod sound_utils;
pub mod throwables;
pub mod ui;
use godot::prelude::*;

struct TheSalmonRiver;

#[gdextension]
unsafe impl ExtensionLibrary for TheSalmonRiver {}
