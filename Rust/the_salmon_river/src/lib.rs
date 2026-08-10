pub mod game_manager;
pub mod game_scripts;
pub mod throwables;
pub mod ui;
// /https://godot-rust.github.io/book/intro/hello-world.html
// https://godot-rust.github.io/docs/gdext/master/godot/classes/index.html
use godot::prelude::*;

struct TheSalmonRiver;

#[gdextension]
unsafe impl ExtensionLibrary for TheSalmonRiver {}
