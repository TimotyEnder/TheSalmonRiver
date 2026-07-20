pub mod player;
// /https://godot-rust.github.io/book/intro/hello-world.html
use godot::prelude::*;

struct TheSalmonRiver;

#[gdextension]
unsafe impl ExtensionLibrary for TheSalmonRiver {}
