pub mod player;
// /https://godot-rust.github.io/book/intro/hello-world.html
// https://godot-rust.github.io/docs/gdext/master/godot/classes/index.html
use godot::prelude::*;

struct TheSalmonRiver;

#[gdextension]
unsafe impl ExtensionLibrary for TheSalmonRiver {}
