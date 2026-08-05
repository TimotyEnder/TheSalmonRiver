pub mod duck_meter_manager;
pub mod player;
pub mod throwable;
pub mod throwable_spawner;
pub mod throwables;
pub mod utils;
// /https://godot-rust.github.io/book/intro/hello-world.html
// https://godot-rust.github.io/docs/gdext/master/godot/classes/index.html
use godot::prelude::*;

struct TheSalmonRiver;

#[gdextension]
unsafe impl ExtensionLibrary for TheSalmonRiver {}
